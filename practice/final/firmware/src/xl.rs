use embedded_hal_async::i2c::I2c;
use embedded_hal_async::digital::Wait;
use lis3dh_async::{Lis3dh, Lis3dhI2C, SlaveAddr, Configuration, Range, Interrupt1, InterruptMode, Mode, DataRate, InterruptConfig, IrqPin1Config, Threshold, Duration, Error};
use crate::board::{XlResources, Irqs};
use embassy_stm32::i2c::{I2c as I2cPeripheral, Master};
use embassy_stm32::mode::Async;
use embassy_stm32::exti::{ExtiInput};
use embassy_stm32::gpio::Pull;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Sender, Receiver};
use embassy_executor::Spawner;
use defmt::{unwrap, warn};

type I2cType = I2cPeripheral<'static, Async, Master>;
type IrqType = ExtiInput<'static>;

pub struct Accel<I: I2c, IRQ: Wait> {
    xl: Lis3dh<Lis3dhI2C<I>>,
    irq: IRQ,
}

#[derive(defmt::Format)]
pub struct Sample {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I: I2c, IRQ: Wait> Accel<I, IRQ> {
    pub async fn new(i2c: I, irq: IRQ) -> Result<Self, Error<I::Error>> {
        let config = Configuration {
            mode: Mode::Normal,
            datarate: DataRate::PowerDown,
            ..Configuration::default()
        };
        let dr = DataRate::Hz_1;

        let mut xl = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, config).await?;

        // Configure the threshold value for interrupt 1 to 1.1g
        let threshold = Threshold::g(Range::default(), 1.1);
        xl.configure_irq_threshold(Interrupt1, threshold).await?;

        // The time in 1/ODR an axis value should be above threshold in order for an
        // interrupt to be raised
        let duration = Duration::miliseconds(dr, 0.0);
        xl.configure_irq_duration(Interrupt1, duration).await?;


        // Raise pin state if interrupt 1 is raised and there is movement
        xl.configure_interrupt_pin(IrqPin1Config {
            ia1_en: true,
            zyxda_en: true,
            ..IrqPin1Config::default()
        }).await?;

        xl.configure_irq_src(
            Interrupt1,
            InterruptMode::Movement,
            InterruptConfig::high_and_low(),
        ).await?;
        xl.set_datarate(dr).await?;



        Ok(Self { xl, irq })
    }

    pub async fn sample(&mut self) -> Result<Sample, Error<I::Error>> {
        let _ = self.irq.wait_for_high().await;
        let sample = self.xl.accel_norm().await?;
        Ok(Sample {
            x: sample.x,
            y: sample.y,
            z: sample.z,
        })
    }
}

pub type SampleStream = Receiver<'static, ThreadModeRawMutex, Sample, 10>;
static STREAM: Channel<ThreadModeRawMutex, Sample, 10> = Channel::new();

pub async fn init(p: XlResources, s: Spawner) -> Result<SampleStream, Error<embassy_stm32::i2c::Error>> {
    let i2c = I2cPeripheral::new(
        p.i2c1,
        p.scl,
        p.sda,
        Irqs,
        p.dma1,
        p.dma2,
        Default::default(),
    );

    let input = ExtiInput::new(p.irq, p.exti, Pull::Down);

    let xl = Accel::new(i2c, input).await?;

    s.spawn(unwrap!(run(xl, STREAM.sender())));
    Ok(STREAM.receiver())
}

#[embassy_executor::task]
async fn run(mut xl: Accel<I2cType, IrqType>, sender: Sender<'static, ThreadModeRawMutex, Sample, 10>) {
    loop {
        match xl.sample().await {
            Ok(sample) => {
                sender.send(sample).await;
            }
            Err(e) => {
                warn!("Error sampling xl: {:?}", e);
            }
        }
    }
}
