#![no_std]
#![no_main]

use embassy_stm32::gpio::Input;
use embassy_stm32::interrupt;
use embassy_stm32::pac;

use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let _b = Input::new(p.PC13, embassy_stm32::gpio::Pull::Down);

    setup();
    loop { }
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
        defmt::info!("Pressed!");
    }
    EXTI.imr(0).modify(|w| w.set_line(pin, true));
}
