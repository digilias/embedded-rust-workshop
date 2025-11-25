/// EXTI1 interrupt handler
#[interrupt]
fn EXTI1() {
    // Clear pending bit
    pac::EXTI.pr(0).write(|w| {
        w.set_line(1, true);
    });

    // Set data ready flag
    critical_section::with(|cs| {
        DATA_READY.borrow(cs).set(true);
    });
}

/// Check if data is ready (non-blocking)
pub fn is_data_ready() -> bool {
    critical_section::with(|cs| {
        let ready = DATA_READY.borrow(cs).get();
        if ready {
            DATA_READY.borrow(cs).set(false); // Clear flag
        }
        ready
    })
}

/// Wait for data ready (blocking)
pub fn wait_for_data_ready() {
    loop {
        if is_data_ready() {
            break;
        }
        cortex_m::asm::wfi(); // Sleep until interrupt
    }
}

fn setup() {
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
}

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
        defmt::info!("Hello!");
    }
    EXTI.imr(0).modify(|w| w.set_line(pin, true));
}
