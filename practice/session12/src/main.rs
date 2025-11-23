#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// TODO: Add extern "C" block to declare C functions
// extern "C" {
//     fn mathlib_init();
//     fn mathlib_cleanup();
//     fn mathlib_crc8(data: *const u8, length: u32, result: *mut u8) -> i32;
//     fn mathlib_sin_fixed(degrees: i32) -> i32;
// }

// TODO: Define error type for C library errors
#[derive(Debug, defmt::Format)]
enum MathLibError {
    NullPointer,
    InvalidLength,
    OutOfRange,
    Unknown(i32),
}

// TODO: Implement safe wrapper for CRC8
// fn calculate_crc8(data: &[u8]) -> Result<u8, MathLibError> {
//     // Your implementation here
// }

// TODO: Implement safe wrapper for sine calculation
// fn calculate_sine(degrees: i32) -> Result<i32, MathLibError> {
//     // Your implementation here
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Session 12: C Library Integration");
    let _p = embassy_stm32::init(Default::default());

    // TODO: Initialize the C library

    loop {
        // TODO: Test CRC8 calculation
        // Example data to calculate CRC for
        let test_data = b"Hello, Embedded Rust!";
        info!("Test data: {:a}", test_data);

        // TODO: Call your safe CRC8 wrapper
        // match calculate_crc8(test_data) {
        //     Ok(crc) => info!("CRC8: 0x{:02x}", crc),
        //     Err(e) => error!("CRC8 error: {:?}", e),
        // }

        // TODO: Test sine calculation
        // for angle in [0, 30, 45, 60, 90, 180, 270] {
        //     match calculate_sine(angle) {
        //         Ok(sin_value) => {
        //             // sin_value is scaled by 1000, so divide for actual value
        //             info!("sin({}) = {}.{:03}", angle, sin_value / 1000, (sin_value.abs() % 1000));
        //         }
        //         Err(e) => error!("Sine calculation error: {:?}", e),
        //     }
        // }

        info!("TODO: Implement C library wrappers and uncomment tests above");

        Timer::after(Duration::from_secs(5)).await;
    }
}
