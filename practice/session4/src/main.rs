#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Error, I2c, Config};
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::{Timer, Duration};
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x18;
const WHOAMI: u8 = 0x0F;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello world!");
    let p = embassy_stm32::init(Default::default());

    let mut config = Config::default();
    config.timeout = Duration::from_secs(2);
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.GPDMA1_CH4,
        p.GPDMA1_CH5,
        config,
    );

    let mut data = [0u8; 1];
    Timer::after(Duration::from_secs(1)).await;

    match i2c.blocking_write_read(ADDRESS, &[WHOAMI], &mut data) {
        Ok(()) => info!("Whoami: {}", data[0]),
        Err(Error::Timeout) => error!("Operation timed out"),
        Err(e) => error!("I2c Error: {:?}", e),
    }
}
