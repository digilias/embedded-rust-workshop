# Session 10: Multiple Executors and Preemption

Learn how to use multiple executors with different priorities for real-time requirements.

## Goal

- Understand task preemption
- Use InterruptExecutor for high-priority tasks
- Learn about RawMutex types and when to use each
- See how the compiler enforces correct mutex usage

## Key Concepts

### InterruptExecutor
```rust
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();

// In main:
let spawner = EXECUTOR_HIGH.start(interrupt::SWI0_EGU0);
spawner.spawn(high_priority_task()).unwrap();
```

### RawMutex Types

- **NoopRawMutex**: Single executor, no preemption
- **ThreadModeRawMutex**: Multiple executors, no interrupt access
- **CriticalSectionRawMutex**: Can be accessed from interrupts

The compiler prevents you from using the wrong one!

## Your Tasks

1. Configure interrupt priority for high-priority executor
2. Start InterruptExecutor
3. Spawn tasks on both executors
4. Observe preemption behavior
5. Try changing RawMutex types and see compiler errors

## Expected Behavior

High-priority tasks can interrupt low-priority tasks mid-execution. This is important for time-critical operations!
