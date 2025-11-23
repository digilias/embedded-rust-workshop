#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Session 8: Async with Interrupts");
    let p = embassy_stm32::init(Default::default());

    // TODO: Create button input with EXTI
    // let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);

    // TODO: Create LED output
    // let led = Output::new(p.PB0, Level::Low, Speed::Low);

    // TODO: Spawn button handler task
    // spawner.spawn(button_handler(button, led)).unwrap();

    info!("TODO: Configure button with EXTI and LED");

    loop {
        info!("Main task");
        Timer::after(Duration::from_secs(5)).await;
    }
}

// TODO: Implement button handler task using async EXTI
// #[embassy_executor::task]
// async fn button_handler(mut button: ExtiInput<'static>, mut led: Output<'static>) {
//     loop {
//         // Wait for button press (falling edge)
//         button.wait_for_falling_edge().await;
//         info!("Button pressed!");
//
//         // Toggle LED
//         led.toggle();
//
//         // Simple debounce delay
//         Timer::after(Duration::from_millis(50)).await;
//     }
// }
