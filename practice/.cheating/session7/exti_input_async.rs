// Session 8 Snippet: Async EXTI with ExtiInput
l
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};

fn main() {
    let _button = Input::new(p.PC13, Pull::Down);
    loop {
        let mut fut = ButtonFuture::new();
        fut.await;
        defmt::info!("Hello from main!");
    }
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
