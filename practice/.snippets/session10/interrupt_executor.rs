// Session 10 Snippet: Using InterruptExecutor for High Priority Tasks

use embassy_executor::{InterruptExecutor, Spawner};
use embassy_stm32::interrupt;
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use core::cell::Cell;
use defmt::info;

// High-priority executor runs in interrupt context
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();

// Shared state - MUST use CriticalSectionRawMutex for cross-executor access
static COUNTER: Mutex<CriticalSectionRawMutex, Cell<u32>> =
    Mutex::new(Cell::new(0));

#[interrupt]
unsafe fn SWI0_EGU0() {
    EXECUTOR_HIGH.on_interrupt()
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());

    // Configure interrupt priority for high-priority executor
    interrupt::SWI0_EGU0.set_priority(Priority::P2);

    // Start high-priority executor
    let high_spawner = EXECUTOR_HIGH.start(interrupt::SWI0_EGU0);

    // Spawn high-priority task
    high_spawner.spawn(high_priority_task()).unwrap();

    // Spawn normal-priority task on main executor
    spawner.spawn(low_priority_task()).unwrap();

    loop {
        Timer::after(Duration::from_secs(10)).await;
    }
}

#[embassy_executor::task]
async fn high_priority_task() {
    loop {
        Timer::after(Duration::from_millis(100)).await;

        // Access shared state
        let counter = COUNTER.lock().await;
        counter.set(counter.get() + 1);
        info!("[HIGH] Counter: {}", counter.get());
    }
}

#[embassy_executor::task]
async fn low_priority_task() {
    loop {
        Timer::after(Duration::from_millis(500)).await;

        let counter = COUNTER.lock().await;
        info!("[LOW] Counter: {}", counter.get());
    }
}

// Key Points:
// - High priority task can preempt low priority task
// - Must use CriticalSectionRawMutex when sharing between executors
// - ThreadModeRawMutex would cause compile error!
