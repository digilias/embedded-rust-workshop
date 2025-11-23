#![no_std]
#![no_main]

use defmt::*;
use stm32_metapac as pac;
use {defmt_rtt as _, panic_probe as _};

// LIS3DH I2C Address
const LIS3DH_ADDR: u8 = 0x18;

// LIS3DH Registers
const WHO_AM_I: u8 = 0x0F;
const EXPECTED_WHO_AM_I: u8 = 0x33;

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Workshop: Embedded Rust with LIS3DH Accelerometer");
    info!("Session 2: Reading WHO_AM_I using PAC");

    // TODO: Initialize I2C peripheral
    // TODO: Read WHO_AM_I register from LIS3DH
    // TODO: Verify it matches EXPECTED_WHO_AM_I

    info!("TODO: Implement I2C initialization and WHO_AM_I read");
    info!("Check snippets/session2/ for reference code");

    loop {
        cortex_m::asm::wfi();
    }
}
