#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _};

// Panic counter stored in a section that survives resets
#[link_section = ".uninit.PANIC_COUNT"]
static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);

// TODO: Implement custom panic handler
// #[panic_handler]
// fn panic_handler(info: &PanicInfo) -> ! {
//     // Increment panic counter
//     PANIC_COUNT.fetch_add(1, Ordering::Relaxed);
//
//     // Log panic information
//     error!("PANIC! Count: {}", PANIC_COUNT.load(Ordering::Relaxed));
//     error!("Info: {}", info);
//
//     // Could save to flash here in real application
//
//     // Reset the system
//     cortex_m::peripheral::SCB::sys_reset();
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Session 11: Custom Panic Handler");
    let _p = embassy_stm32::init(Default::default());

    let count = PANIC_COUNT.load(Ordering::Relaxed);
    if count > 0 {
        warn!("Device has panicked {} times since last full reset", count);
    }

    info!("TODO: Implement custom panic handler");
    info!("Try uncommenting the panic below to test");

    loop {
        info!("Running normally...");
        Timer::after(Duration::from_secs(2)).await;

        // Uncomment to test panic handler:
        // panic!("Test panic!");
    }
}
