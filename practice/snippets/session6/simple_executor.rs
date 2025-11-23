// Session 6 Snippet: Simple Executor Implementation
// Educational example - not for production use!

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub struct SimpleExecutor {
    // In a real executor, you'd store tasks in a queue
    // For simplicity, this example assumes tasks are externally managed
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {}
    }

    /// Run a single future to completion
    pub fn block_on<F: Future>(&mut self, mut future: F) -> F::Output {
        // Pin the future
        let mut future = unsafe { Pin::new_unchecked(&mut future) };

        // Create a dummy waker
        let waker = dummy_waker();
        let mut context = Context::from_waker(&waker);

        // Poll until ready
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    // In a real executor, we'd sleep or wait for events
                    // For this simple version, we just loop (busy-wait)
                }
            }
        }
    }
}

// Dummy waker implementation
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone, no_op, no_op, no_op);

    RawWaker::new(core::ptr::null(), &VTABLE)
}

// Example usage:
//
// let mut executor = SimpleExecutor::new();
// let result = executor.block_on(async {
//     // Your async code here
//     42
// });
