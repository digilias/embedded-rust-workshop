use embedded_hal_async::i2c::I2c;
use embedded_hal_async::digital::Wait;
use lis3dh_async::{Lis3dh, Lis3dhI2C, SlaveAddr, Configuration, Range, Interrupt1, InterruptMode, Mode, DataRate, InterruptConfig, IrqPin1Config, Threshold, Duration};
use crate::board::{XlResources, Irqs};
use embassy_stm32::i2c::{I2c as I2cPeripheral, Master};
use embassy_stm32::mode::Async;
use embassy_stm32::exti::{ExtiInput};
use embassy_stm32::gpio::Pull;

type I2cType = I2cPeripheral<'static, Async, Master>;

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
    pub async fn new(i2c: I, irq: IRQ) -> Self {
        let config = Configuration {
            mode: Mode::Normal,
            datarate: DataRate::PowerDown,
            ..Configuration::default()
        };
        let dr = DataRate::Hz_1;
        let mut xl = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, config).await.unwrap();
        let threshold = Threshold::g(Range::default(), 1.1);

        // Configure the threshold value for interrupt 1 to 1.1g
        xl.configure_irq_threshold(Interrupt1, threshold).await.unwrap();

        // The time in 1/ODR an axis value should be above threshold in order for an
        // interrupt to be raised
        let duration = Duration::miliseconds(dr, 0.0);
        xl.configure_irq_duration(Interrupt1, duration).await.unwrap();


        xl.configure_interrupt_pin(IrqPin1Config {
            // Raise if interrupt 1 is raised
            ia1_en: true,
            zyxda_en: true,
            // Disable for all other interrupts
            ..IrqPin1Config::default()
        }).await.unwrap();
        xl.configure_irq_src(
            Interrupt1,
            InterruptMode::Movement,
            InterruptConfig::high_and_low(),
        ).await.unwrap();
        xl.set_datarate(dr).await.unwrap();
        Self { xl, irq }
    }

    pub async fn sample(&mut self) -> Result<Sample, ()> {
        self.irq.wait_for_high().await;
        let sample = self.xl.accel_norm().await.unwrap();
        Ok(Sample {
            x: sample.x,
            y: sample.y,
            z: sample.z,
        })
    }
}

pub async fn init(p: XlResources) -> Accel<I2cType, ExtiInput<'static>> {
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
    Accel::new(i2c, input).await
}
