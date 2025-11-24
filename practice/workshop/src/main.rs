#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::pac as _;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello!");
    loop {}
}
