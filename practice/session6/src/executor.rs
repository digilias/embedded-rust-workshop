use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use defmt::*;
use heapless::Vec;

const MAX_TASKS: usize = 8;

type BoxedFuture = Pin<&'static mut dyn Future<Output = ()>>;

pub struct SimpleExecutor {
    tasks: Vec<BoxedFuture, MAX_TASKS>,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    // TODO: Implement spawn method
    // pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
    //     // Box the future and pin it
    //     // Add to tasks queue
    //     // Hint: You'll need to use Box::leak for 'static lifetime in no_std
    //     todo!()
    // }

    // TODO: Implement run method (busy-loop executor)
    // pub fn run(&mut self) -> ! {
    //     loop {
    //         // For each task:
    //         // 1. Create a waker (use dummy_waker() helper below)
    //         // 2. Create a Context with the waker
    //         // 3. Poll the future
    //         // 4. If Ready, remove from queue
    //         // 5. If Pending, keep in queue for next iteration
    //
    //         if self.tasks.is_empty() {
    //             info!("All tasks complete");
    //             loop {
    //                 cortex_m::asm::wfi();
    //             }
    //         }
    //     }
    // }
}

// TODO: Implement a dummy waker
// For a simple busy-loop executor, the waker doesn't need to do anything
// because we continuously poll all tasks
//
// fn dummy_waker() -> Waker {
//     // Create a RawWaker with noop functions
//     // Wrap in Waker
//     todo!()
// }

// Helper functions for the dummy waker
unsafe fn dummy_clone(_: *const ()) -> RawWaker {
    dummy_raw_waker()
}

unsafe fn dummy_wake(_: *const ()) {
    // Do nothing
}

unsafe fn dummy_wake_by_ref(_: *const ()) {
    // Do nothing
}

unsafe fn dummy_drop(_: *const ()) {
    // Do nothing
}

fn dummy_raw_waker() -> RawWaker {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        dummy_clone,
        dummy_wake,
        dummy_wake_by_ref,
        dummy_drop,
    );

    RawWaker::new(core::ptr::null(), &VTABLE)
}
