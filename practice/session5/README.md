# Session 5: Handling Interrupts

This session demonstrates how to configure and handle hardware interrupts on ARM Cortex-M microcontrollers.

## Goal

Learn interrupt handling by:
- Configuring EXTI (External Interrupt) for GPIO
- Using the NVIC (Nested Vectored Interrupt Controller)
- Sharing data safely between interrupt and main code
- Understanding critical sections and interrupt safety

## What's Provided

- `src/main.rs`: Main program with button configuration and shared state setup
- Skeleton code with TODO markers for EXTI configuration and interrupt handler

## Your Tasks

### 1. Configure EXTI for Button Press
Configure the EXTI peripheral to trigger on button press (PC13):

```rust
// Enable SYSCFG clock
pac::RCC.apb3enr().modify(|w| {
    w.set_syscfgen(true);
});

// Select PC13 as EXTI13 source
pac::EXTI.exticr(3).modify(|w| {
    w.set_exti(13, pac::exti::vals::Exticr::PC);
});

// Configure falling edge trigger (button press)
pac::EXTI.ftsr(0).modify(|w| {
    w.set_line(13, true);
});

// Unmask EXTI13
pac::EXTI.imr(0).modify(|w| {
    w.set_line(13, true);
});
```

### 2. Enable NVIC Interrupt
Enable the EXTI13 interrupt in the NVIC:

```rust
use cortex_m::peripheral::NVIC;

unsafe {
    NVIC::unmask(pac::Interrupt::EXTI13);
}
```

### 3. Implement Interrupt Handler
Create the interrupt handler function:

```rust
#[interrupt]
fn EXTI13() {
    // Clear pending bit - REQUIRED!
    pac::EXTI.pr(0).write(|w| {
        w.set_line(13, true);
    });

    // Increment counter
    critical_section::with(|cs| {
        let counter = BUTTON_PRESSED.borrow(cs);
        counter.set(counter.get() + 1);
    });

    info!("Button interrupt!");
}
```

Don't forget to import the interrupt macro:
```rust
use cortex_m_rt::interrupt;
```

## Key Concepts

### EXTI (External Interrupt/Event Controller)
- Generates interrupts from GPIO pins
- Configurable trigger: rising edge, falling edge, or both
- 16 EXTI lines, each can be mapped to different GPIO ports

### NVIC (Nested Vectored Interrupt Controller)
- Manages all interrupts in Cortex-M
- Enable/disable specific interrupts
- Set interrupt priorities
- Pending and active status

### Critical Sections
```rust
static SHARED: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

// In interrupt or main:
critical_section::with(|cs| {
    let value = SHARED.borrow(cs);
    value.set(value.get() + 1);
});
```

Critical sections ensure atomic access to shared data by temporarily disabling interrupts.

### Interrupt Macro
```rust
#[interrupt]
fn EXTI13() {
    // Handler code
}
```

The `#[interrupt]` macro (from cortex-m-rt):
- Creates proper interrupt vector
- Ensures correct function signature
- Sets up stack frame

### Important Rules

1. **Always clear pending bits**: Forgetting this causes infinite interrupt loops
2. **Keep handlers short**: Long handlers delay other interrupts
3. **Use critical sections**: For safe shared data access
4. **Avoid blocking**: No delays or long computations in handlers

## Building and Running

```bash
cargo build
cargo run
```

Expected output:
```
INFO  Session 5: Handling Interrupts
INFO  Waiting for button press...
<Press button>
INFO  Button interrupt!
INFO  Button pressed 1 times
<Press button again>
INFO  Button interrupt!
INFO  Button pressed 2 times
```

## Debugging Tips

### Interrupt Not Firing?
- Check EXTI configuration (correct pin, correct trigger edge)
- Verify NVIC is unmasked
- Ensure GPIO is configured as input
- Check if button has pull-up/pull-down

### Infinite Interrupts?
- Forgot to clear pending bit in handler
- Wrong edge configuration (e.g., both edges when you want one)

### Counter Not Updating?
- Not using critical sections correctly
- Race condition between interrupt and main

## Extension Challenges

1. **Add LED Toggle**: Toggle an LED in the interrupt handler
2. **Debouncing**: Implement software debouncing for button
3. **Multiple Interrupts**: Handle multiple EXTI lines (e.g., multiple buttons)
4. **Interrupt Priority**: Configure different priorities for interrupts
5. **Edge Detection**: Support both rising and falling edges
6. **Timing**: Measure time between button presses using SysTick timer

## Comparison with Async (Preview)

**With Interrupts (Session 5)**:
```rust
#[interrupt]
fn EXTI13() {
    // Handle immediately
}
```

**With Async (Session 7+)**:
```rust
let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);
button.wait_for_falling_edge().await;
// Handle after event
```

Async provides a higher-level abstraction but still uses interrupts under the hood!
