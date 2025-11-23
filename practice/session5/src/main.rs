#![no_std]
#![no_main]

use core::cell::Cell;
use critical_section::Mutex;
use defmt::*;
use pac::gpio::vals;
use stm32_metapac as pac;
use {defmt_rtt as _, panic_probe as _};

// Shared state between interrupt and main
// Must use Mutex + Cell for interior mutability in interrupt context
static BUTTON_PRESSED: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Session 5: Handling Interrupts");

    // Enable clocks
    pac::RCC.ahb2enr().modify(|w| {
        w.set_gpiocen(true); // For button (PC13)
        w.set_gpioben(true); // For LED (optional)
    });

    // Configure PC13 as input for button (typically user button on Nucleo)
    pac::GPIOC.moder().modify(|w| {
        w.set_moder(13, vals::Moder::INPUT);
    });

    pac::GPIOC.pupdr().modify(|w| {
        w.set_pupdr(13, vals::Pupdr::PULLUP);
    });

    // TODO: Configure EXTI (External Interrupt) for PC13
    // 1. Select PC13 as EXTI13 source in EXTI configuration register
    // 2. Configure trigger (rising/falling/both edges)
    // 3. Unmask EXTI13 interrupt

    // Hint: Use pac::EXTI and pac::SYSCFG

    // TODO: Enable EXTI13 interrupt in NVIC
    // Use cortex_m::peripheral::NVIC

    info!("TODO: Configure EXTI interrupt for button");
    info!("Interrupt handler should increment counter when button is pressed");

    let mut last_count = 0;
    loop {
        // Read button press count from shared state
        let count = critical_section::with(|cs| BUTTON_PRESSED.borrow(cs).get());

        if count != last_count {
            info!("Button pressed {} times", count);
            last_count = count;
        }

        cortex_m::asm::wfi(); // Wait for interrupt
    }
}

// TODO: Implement EXTI13 interrupt handler
// This function is called when the EXTI13 interrupt fires
//
// #[interrupt]
// fn EXTI13() {
//     // Clear the pending interrupt flag
//     // TODO: Clear EXTI13 pending bit
//
//     // Increment button press counter
//     critical_section::with(|cs| {
//         let counter = BUTTON_PRESSED.borrow(cs);
//         counter.set(counter.get() + 1);
//     });
//
//     info!("Interrupt triggered!");
// }
