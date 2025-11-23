#![no_std]
#![no_main]

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use defmt::*;
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};

mod executor;
mod timer_future;

use executor::SimpleExecutor;
use timer_future::TimerFuture;

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Session 6: Building a Simple Async Executor");

    // Create executor
    let mut executor = SimpleExecutor::new();

    // Spawn tasks
    executor.spawn(example_task_1());
    executor.spawn(example_task_2());

    // Run executor
    info!("Starting executor...");
    executor.run();

    // Should never reach here
    loop {
        cortex_m::asm::wfi();
    }
}

async fn example_task_1() {
    info!("Task 1: Started");

    for i in 0..5 {
        info!("Task 1: Step {}", i);
        TimerFuture::new(100).await;
    }

    info!("Task 1: Complete");
}

async fn example_task_2() {
    info!("Task 2: Started");

    for i in 0..3 {
        info!("Task 2: Step {}", i);
        TimerFuture::new(200).await;
    }

    info!("Task 2: Complete");
}
