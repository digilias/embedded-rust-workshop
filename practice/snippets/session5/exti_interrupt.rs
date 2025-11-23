// Session 5 Snippet: EXTI Interrupt Handler for LIS3DH

use core::cell::Cell;
use critical_section::Mutex;
use cortex_m_rt::interrupt;
use stm32_metapac as pac;

// Shared state: set to true when data is ready
static DATA_READY: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

/// Initialize EXTI for LIS3DH INT1 pin (e.g., PC1)
pub fn init_exti_for_lis3dh() {
    // Assuming INT1 is connected to PC1

    // Enable GPIOC clock
    pac::RCC.ahb2enr().modify(|w| {
        w.set_gpiocen(true);
    });

    // Configure PC1 as input
    pac::GPIOC.moder().modify(|w| {
        w.set_moder(1, pac::gpio::vals::Moder::INPUT);
    });

    // Enable SYSCFG clock (needed for EXTI)
    pac::RCC.apb3enr().modify(|w| {
        w.set_syscfgen(true);
    });

    // Select PC1 as EXTI1 source
    pac::EXTI.exticr(0).modify(|w| {
        w.set_exti(1, pac::exti::vals::Exticr::PC);
    });

    // Configure rising edge trigger (LIS3DH INT1 goes high when active)
    pac::EXTI.rtsr(0).modify(|w| {
        w.set_line(1, true);
    });

    // Unmask EXTI1
    pac::EXTI.imr(0).modify(|w| {
        w.set_line(1, true);
    });

    // Enable EXTI1 interrupt in NVIC
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI1);
    }
}

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
