#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::i2c::{Error, I2c, Config};
use embassy_time::Duration;

const ADDRESS: u8 = 0x18;
const WHOAMI: u8 = 0x0F;

use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    let mut config = Config::default();
    config.timeout = Duration::from_secs(2);
    let mut i2c = I2c::new_blocking(
        p.I2C1,
        p.PB8,
        p.PB9,
        config,
    );

    // Use it to read the register
    let mut data = [0u8; 1];
    match i2c.blocking_write_read(ADDRESS, &[WHOAMI], &mut data) {
        Ok(()) => info!("Whoami: {}", data[0]),
        Err(Error::Timeout) => error!("Operation timed out"),
        Err(e) => error!("I2c Error: {:?}", e),
    }
    loop {}
}
