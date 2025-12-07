// Session 7 Snippet: Embassy Executor Tasks

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use defmt::info;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Embassy Executor Example");
    let _p = embassy_stm32::init(Default::default());

    // Spawn multiple tasks
    spawner.spawn(blinker()).unwrap();
    spawner.spawn(sensor_reader()).unwrap();
    spawner.spawn(logger()).unwrap();

    // Main task can also do work
    loop {
        info!("[main] Running");
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn blinker() {
    info!("[blinker] Started");
    loop {
        info!("[blinker] Tick");
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn sensor_reader() {
    info!("[sensor] Started");
    loop {
        // Simulate reading sensor
        info!("[sensor] Reading...");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn logger() {
    let mut count = 0;
    loop {
        info!("[logger] Count: {}", count);
        count += 1;
        Timer::after(Duration::from_secs(2)).await;
    }
}
