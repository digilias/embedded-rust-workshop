# Session 7: Embassy Executor

This session introduces the embassy-executor, a production-ready async executor for embedded systems.

## Goal

Learn to use embassy-executor by:
- Using `#[embassy_executor::main]` for the main task
- Spawning concurrent tasks with `#[embassy_executor::task]`
- Using embassy-time for async delays
- Understanding task lifecycle and the Spawner

## Your Tasks

1. **Create LED output**: Initialize GPIO pin for LED
2. **Implement blinker task**: Toggle LED every 500ms
3. **Implement counter task**: Print counter every second
4. **Spawn both tasks**: Use spawner to run tasks concurrently

## Key Concepts

### Task Macro
```rust
#[embassy_executor::task]
async fn my_task() {
    // Task implementation
}
```

### Spawning
```rust
spawner.spawn(my_task()).unwrap();
```

### Embassy-Time
```rust
Timer::after(Duration::from_secs(1)).await;
```

## Building and Running

```bash
cargo run
```

Expected: LED blinking while counter increments simultaneously.
