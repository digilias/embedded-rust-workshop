use embedded_hal_async::i2c::I2c;
use lis3dh_async::{Lis3dh, Lis3dhI2C, SlaveAddr};
use crate::board::{XlResources, Irqs};
use embassy_stm32::i2c::{I2c as I2cPeripheral, Master};
use embassy_stm32::mode::Async;

type I2cType = I2cPeripheral<'static, Async, Master>;

pub struct Accel<I: I2c> {
    xl: Lis3dh<Lis3dhI2C<I>>,
}

pub struct Sample {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I: I2c> Accel<I> {
    pub async fn new(i2c: I) -> Self {
        let xl = Lis3dh::new_i2c(i2c, SlaveAddr::Default).await.unwrap();
        Self { xl }
    }

    pub async fn sample(&mut self) -> Result<Sample, ()> {
        let sample = self.xl.accel_norm().await.unwrap();
        Ok(Sample {
            x: sample.x,
            y: sample.y,
            z: sample.z,
        })
    }
}

pub async fn init(p: XlResources) -> Accel<I2cType> {
    let i2c = I2cPeripheral::new(
        p.i2c1,
        p.scl,
        p.sda,
        Irqs,
        p.dma1,
        p.dma2,
        Default::default(),
    );
    
    Accel::new(i2c).await
}
