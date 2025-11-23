---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 11: Panicking, logging and debugging

* **Goal:** How to customize panic handlers and how logging works.

---
# The panic handler

```rust
// Called
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
}
```
---
# Custom handler

* You can override specific handlers

```rust
#[unsafe(no_mangle)]
pub extern "C" fn HardFault() -> ! {
    // do something interesting here
    loop {}
}
```

---

# Existing crates

* `panic-abort` - performs `abort` instruction on panic
* `panic-reset` - immediately does a reset
* `panic-halt` - halt in infinite loop
* `panic-itm` - uses ARM ITM to log panic
* `panic-persist` - persist panic messages in RAM
* `panic-semihosting` - logged to host using semihosting
* `panic-probe` - logs panic message over RTT

---

# Using an existing panic handler

```rust
use panic_halt as _;
```

* Imports the function
* No more work needed

---

# Why create your own

* Log the information you care about
* Ensuring system state is properly reset

--- 
# Example

```rust
unsafe extern "C" fn RealHardFault(sp: *const u8) -> ! {
    // Try to save the time so it survives across the crash...
    crate::time::persist_save();

    // If logbuf is already frozen, don't log anything.
    if log::is_frozen() {
        cortex_m::peripheral::SCB::sys_reset();
    }

    // Force-untake the logger, so the following error gets logged no matter what
    // even if the hardfault ocurred while logging something else
    log::force_untake();

    // Dump RAM to external flash
    #[cfg(feature = "ramdump")]
    crate::ramdump::dump();

    #[cfg(feature = "trace")]
    ak::trace::print();

    error!("HardFault! stack dump: {=[u8]:x}", slice::from_raw_parts(sp, 1024));

    // Freeze logbuf
    log::freeze();

    cortex_m::peripheral::SCB::sys_reset();
}
```

---

# Logging

* When developing: using RTT (probe-rs) to transfer logs to host
* In production: storing a buffer in RAM

---

# `defmt` - deferred formatting

* A logging library commonly used
* Each unique string literal translated to a symbol
* Symbol mapping derived at compile time and stored in the ELF
* Logs are encoded using symbols
* Logs are decoded using symbols 

# `defmt` - example

```rust
let val = 42;
defmt::info!("Hello {}", val);
```

```rust

```

# debugging
