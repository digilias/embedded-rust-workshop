use embedded_hal_async::i2c::I2c;
use embedded_hal_async::digital::Wait;
use embedded_hal::digital::InputPin;
use lis3dh_async::{Lis3dh, Lis3dhI2C, SlaveAddr, Configuration, Interrupt1, InterruptMode, Mode, DataRate, InterruptConfig, IrqPin1Config, Error};
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

pub struct Accel<I: I2c, IRQ: Wait + InputPin> {
    xl: Lis3dh<Lis3dhI2C<I>>,
    irq: IRQ,
    filter: LowpassFilter,
}


#[derive(Clone, Copy, defmt::Format)]
pub struct Sample {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I: I2c, IRQ: Wait + InputPin> Accel<I, IRQ> {
    pub async fn new(i2c: I, irq: IRQ) -> Result<Self, Error<I::Error>> {
        let config = Configuration {
            mode: Mode::HighResolution,
            datarate: DataRate::PowerDown,
            ..Configuration::default()
        };

        let mut xl = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, config).await?;

        let dr = DataRate::Hz_100;
        xl.set_datarate(dr).await?;

        xl.configure_irq_src(
            Interrupt1,
            InterruptMode::Position,
            InterruptConfig::high_and_low(),
        ).await?;

        // Raise pin state if interrupt 1 is raised and there is movement
        xl.configure_interrupt_pin(IrqPin1Config {
            zyxda_en: true,
            ..IrqPin1Config::default()
        }).await?;


        Ok(Self {
            xl,
            irq,
            filter: LowpassFilter::new(0.1), // Lower value -> smoother but more delay
        })
    }

    pub async fn sample(&mut self) -> Result<Sample, Error<I::Error>> {
        let _ = self.irq.wait_for_high().await;
        let raw_sample = self.xl.accel_norm().await?;
        let raw_sample = Sample {
            x: raw_sample.x,
            y: raw_sample.y,
            z: raw_sample.z,
        };
        let filtered_sample = self.filter.apply(raw_sample);
        Ok(filtered_sample)
    }
}


struct LowpassFilter {
    filter_state: Option<Sample>,
    alpha: f32,
}

impl LowpassFilter {
    fn new(alpha: f32) -> Self {
        Self {
            filter_state: None,
            alpha,
        }
    }

    fn apply(&mut self, raw_sample: Sample) -> Sample {
        match self.filter_state {
            None => {
                // First sample, initialize filter state
                self.filter_state = Some(raw_sample);
                raw_sample
            }
            Some(prev) => {
                // Apply exponential moving average: filtered = alpha * new + (1 - alpha) * prev
                let filtered = Sample {
                    x: self.alpha * raw_sample.x + (1.0 - self.alpha) * prev.x,
                    y: self.alpha * raw_sample.y + (1.0 - self.alpha) * prev.y,
                    z: self.alpha * raw_sample.z + (1.0 - self.alpha) * prev.z,
                };
                self.filter_state = Some(filtered);
                filtered
            }
        }
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

    let input = ExtiInput::new(p.irq, p.exti, Pull::None, Irqs);

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
