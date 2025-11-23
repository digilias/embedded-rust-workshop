#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Session 7: Embassy Executor");
    let p = embassy_stm32::init(Default::default());

    // TODO: Create LED output pin
    // let led = Output::new(p.PB0, Level::Low, Speed::Low);

    // TODO: Spawn blinker task
    // spawner.spawn(blinker(led)).unwrap();

    // TODO: Spawn counter task
    // spawner.spawn(counter()).unwrap();

    info!("TODO: Create LED and spawn tasks");

    loop {
        info!("Main task running");
        Timer::after(Duration::from_secs(5)).await;
    }
}

// TODO: Implement LED blinker task
// #[embassy_executor::task]
// async fn blinker(mut led: Output<'static>) {
//     loop {
//         led.set_high();
//         Timer::after(Duration::from_millis(500)).await;
//         led.set_low();
//         Timer::after(Duration::from_millis(500)).await;
//     }
// }

// TODO: Implement counter task
// #[embassy_executor::task]
// async fn counter() {
//     let mut count = 0;
//     loop {
//         info!("Count: {}", count);
//         count += 1;
//         Timer::after(Duration::from_secs(1)).await;
//     }
// }
