#![no_std]
#![no_main]

use embassy_stm32::interrupt;
use embassy_stm32::pac;
use core::pin::Pin;
use embassy_sync::waitqueue::AtomicWaker;
use core::task::{RawWaker, RawWakerVTable, Waker, Poll, Context};
use embassy_stm32::gpio::{Input, Pull};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut executor = SimpleExecutor::new();
    executor.block_on(async {
        let p = embassy_stm32::init(Default::default());

    let _button = Input::new(p.PC13, Pull::Down);
    loop {
        let fut = ButtonFuture::new();
        fut.await;
        defmt::info!("Button pressed!");
    }
    });
    loop {}
}
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

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);

    RawWaker::new(core::ptr::null(), &VTABLE)
}

struct ButtonFuture;
impl ButtonFuture {
    pub fn new() -> Self {
        // Pin: 13 port 2 rising false falling true drop true
        let pin = 13;
        let port = 2;
        let rising = true;
        let falling = false;

        use pac::EXTI;
        EXTI.exticr(pin / 4).modify(|w| w.set_exti(pin % 4, port));
        EXTI.rtsr(0).modify(|w| w.set_line(pin, rising));
        EXTI.ftsr(0).modify(|w| w.set_line(pin, falling));
        EXTI.rpr(0).write(|w| w.set_line(pin, true));
        EXTI.fpr(0).write(|w| w.set_line(pin, true));
        EXTI.imr(0).modify(|w| w.set_line(pin, true));
        Self
    }
}
impl Future for ButtonFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use pac::EXTI;
        WAKER.register(cx.waker());

        let pin = 13;
        let imr = EXTI.imr(0).read();
        if !imr.line(pin) {
            EXTI.imr(0).modify(|w| w.set_line(pin, true));
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();

#[interrupt]
unsafe fn EXTI13() {
    use pac::exti::regs::Lines;
    use pac::EXTI;

    let bits = EXTI.rpr(0).read().0 | EXTI.fpr(0).read().0;
    let bits = bits & 0x0000FFFF;
    EXTI.imr(0).modify(|w| w.0 &= !bits);

    EXTI.rpr(0).write_value(Lines(bits));
    EXTI.fpr(0).write_value(Lines(bits));

    let pin = 13;
    let imr = EXTI.imr(0).read();
    if !imr.line(pin) {
        WAKER.wake();
    }
}
