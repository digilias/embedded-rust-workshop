# Session 8: Combining Async with Interrupts

Learn how embassy combines async/await with interrupt-driven programming.

## Goal

- Use `ExtiInput` for async interrupt handling
- Understand how `.await` works with interrupts
- See the benefits of async over raw interrupt handlers

## Key Concepts

### ExtiInput
```rust
let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);
button.wait_for_falling_edge().await;
```

Embassy handles the interrupt internally and wakes your task!

## Your Tasks

1. Create `ExtiInput` for button
2. Create `Output` for LED
3. Implement task that waits for button press and toggles LED
4. Add debouncing delay

## Comparison

**Session 5 (raw interrupts)**:
- Manual interrupt handler
- Shared state with Mutex
- Complex synchronization

**Session 8 (async)**:
- Simple `.await` syntax
- No manual interrupt handling
- Embassy manages waking and state

Much cleaner code with same functionality!
