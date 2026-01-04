// Session 11 Snippet: Custom Panic Handler

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::error;

// Panic counter in uninitialized memory section (survives soft resets)
#[link_section = ".uninit.PANIC_COUNT"]
static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Increment panic counter
    let count = PANIC_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    // Log panic information
    error!("=== PANIC #{} ===", count);
    error!("Info: {}", info);

    // Could save additional state here:
    // - Last sensor reading
    // - Current task state
    // - Stack dump

    // In production, you might:
    // 1. Save panic info to flash
    // 2. Dump critical RAM sections
    // 3. Send alert over network (if available)

    // Reset the system
    cortex_m::peripheral::SCB::sys_reset();
}

// Check panic count at startup
pub fn check_panic_history() {
    let count = PANIC_COUNT.load(Ordering::Relaxed);
    if count > 0 {
        defmt::warn!("Device has panicked {} times", count);

        // Could implement panic loop detection:
        if count > 5 {
            defmt::error!("Too many panics! Entering safe mode");
            // Enter safe mode (disable features, etc.)
        }
    }
}

// Reset panic count (call after addressing issue)
pub fn reset_panic_count() {
    PANIC_COUNT.store(0, Ordering::Relaxed);
    defmt::info!("Panic count reset");
}
