#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x5F;
const WHOAMI: u8 = 0x0F;

#[cortex_m_rt::entry]
fn main() {
    info!("Hello world!");
}
