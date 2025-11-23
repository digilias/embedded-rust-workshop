# Session 9: Synchronization Patterns

Learn different ways to share data and communicate between async tasks.

## Goal

- Use Channel for producer-consumer patterns
- Use Signal for event notification
- Use Mutex for shared state
- Understand when to use each primitive

## Primitives

### Channel
```rust
static CHANNEL: Channel<NoopRawMutex, i32, 10> = Channel::new();

// Producer
channel.send(value).await;

// Consumer
let value = channel.receive().await;
```

### Signal
```rust
static SIGNAL: Signal<NoopRawMutex, Command> = Signal::new();

// Sender
SIGNAL.signal(cmd);

// Receiver
let cmd = SIGNAL.wait().await;
```

### Mutex
```rust
static MUTEX: Mutex<NoopRawMutex, Cell<u32>> = Mutex::new(Cell::new(0));

let value = MUTEX.lock().await;
```

## Your Tasks

1. Implement producer/consumer with Channel
2. Implement command signaling with Signal
3. Experiment with Mutex for shared counter
4. Observe the different synchronization behaviors

## When to Use What

- **Channel**: Producer-consumer, data streaming
- **Signal**: Event notifications, simple commands
- **Mutex**: Protecting shared mutable state
