#![no_std]
#![no_main]

use core::cell::Cell;
use defmt::*;
use embassy_executor::{InterruptExecutor, Spawner};
use embassy_stm32::interrupt;
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex};
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// Shared counter - must use CriticalSectionRawMutex for multi-executor
static SHARED_COUNTER: Mutex<CriticalSectionRawMutex, Cell<u32>> =
    Mutex::new(Cell::new(0));

// High priority executor
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();

#[embassy_executor::task]
async fn high_priority_task() {
    loop {
        Timer::after(Duration::from_millis(100)).await;

        // Access shared state
        let counter = SHARED_COUNTER.lock().await;
        counter.set(counter.get() + 1);

        info!("[HIGH] Counter: {}", counter.get());
    }
}

#[embassy_executor::task]
async fn low_priority_task() {
    loop {
        Timer::after(Duration::from_millis(500)).await;

        let counter = SHARED_COUNTER.lock().await;
        info!("[LOW] Counter: {}", counter.get());
    }
}

#[interrupt]
unsafe fn SWI0_EGU0() {
    EXECUTOR_HIGH.on_interrupt()
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Session 10: Multiple Executors and Preemption");
    let _p = embassy_stm32::init(Default::default());

    // TODO: Configure interrupt priorities
    // interrupt::SWI0_EGU0.set_priority(Priority::P2);

    // TODO: Start high priority executor
    // let high_spawner = EXECUTOR_HIGH.start(interrupt::SWI0_EGU0);
    // high_spawner.spawn(high_priority_task()).unwrap();

    // Spawn low priority task on main executor
    spawner.spawn(low_priority_task()).unwrap();

    info!("TODO: Configure high priority executor");
    info!("Observe: High priority task can preempt low priority");

    loop {
        info!("[MAIN] Running");
        Timer::after(Duration::from_secs(2)).await;
    }
}
