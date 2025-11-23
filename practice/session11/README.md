# Session 11: Custom Panic Handler and Logging

Learn to create custom panic handlers that save diagnostic information.

## Goal

- Implement a custom panic handler
- Persist panic information across resets
- Understand panic handler requirements
- Learn about defmt logging internals

## Your Tasks

### 1. Implement Panic Handler
```rust
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // 1. Increment counter
    PANIC_COUNT.fetch_add(1, Ordering::Relaxed);

    // 2. Log information
    error!("PANIC: {}", info);

    // 3. Save to persistent storage (flash/RAM)

    // 4. Reset or halt
    cortex_m::peripheral::SCB::sys_reset();
}
```

### 2. Use Persistent Memory
The `.uninit` section survives soft resets:
```rust
#[link_section = ".uninit.PANIC_COUNT"]
static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);
```

Add to `memory.x` or linker script:
```ld
.uninit (NOLOAD) : {
    *(.uninit.*)
} > RAM
```

### 3. Test the Handler
- Uncomment the `panic!()` call
- Observe counter incrementing across resets
- Add more diagnostic information

## Advanced Ideas

1. **Save stack trace**: Capture and log stack information
2. **Save to flash**: Write panic info to non-volatile memory
3. **Dump RAM**: Save critical RAM sections
4. **Breadcrumb logging**: Maintain circular buffer of events
5. **Watchdog integration**: Catch hangs before they become permanent

## Real-World Usage

Custom panic handlers are essential for:
- Debugging field issues
- Understanding crash patterns
- Implementing fail-safe recovery
- Meeting safety requirements

## Extension: defmt Logging

### How defmt Works
- String literals become compile-time symbols
- Only symbols (not strings) sent over RTT
- Host-side decoder reconstructs messages
- Minimal runtime overhead

### Log Levels
```rust
trace!("Very detailed");
debug!("Development info");
info!("General information");
warn!("Warning condition");
error!("Error occurred");
```

Control via `DEFMT_LOG` environment variable.
