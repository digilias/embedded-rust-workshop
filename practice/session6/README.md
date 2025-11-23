# Session 6: Async Fundamentals - Building an Executor

This session teaches async/await fundamentals by implementing a simple executor from scratch.

## Goal

Understand async Rust by:
- Implementing the Future trait
- Creating a simple poll-based executor
- Understanding wakers and Context
- Learning how async/await is desugared

## What's Provided

- `src/main.rs`: Entry point with two example async tasks
- `src/executor.rs`: Skeleton for a simple busy-loop executor
- `src/timer_future.rs`: Skeleton for a simple timer future

## Your Tasks

### 1. Implement TimerFuture
In `src/timer_future.rs`, implement the `Future` trait:

```rust
impl Future for TimerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polls_remaining == 0 {
            Poll::Ready(())
        } else {
            self.polls_remaining -= 1;
            Poll::Pending
        }
    }
}
```

### 2. Implement Executor
In `src/executor.rs`:

**Create dummy waker:**
```rust
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
```

**Implement spawn (simplified for learning):**
```rust
pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
    // For no_std, we need to box and leak for 'static lifetime
    // In real code, use proper allocators
    use core::mem;

    let boxed: Pin<Box<dyn Future<Output = ()>>> = Box::pin(future);
    let leaked: Pin<&'static mut dyn Future<Output = ()>> =
        unsafe { mem::transmute(boxed) };

    self.tasks.push(leaked).ok();
}
```

**Implement run:**
```rust
pub fn run(&mut self) -> ! {
    loop {
        let mut i = 0;
        while i < self.tasks.len() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);

            match self.tasks[i].as_mut().poll(&mut context) {
                Poll::Ready(()) => {
                    // Task complete, remove from queue
                    self.tasks.swap_remove(i);
                }
                Poll::Pending => {
                    // Task not ready, move to next
                    i += 1;
                }
            }
        }

        if self.tasks.is_empty() {
            info!("All tasks complete");
            loop { cortex_m::asm::wfi(); }
        }
    }
}
```

### 3. Test Your Executor
Build and run. You should see tasks execute interleaved based on their timer durations.

## Key Concepts

### The Future Trait
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),    // Future completed
    Pending,     // Future not ready yet
}
```

### How async/await Works

**This async function:**
```rust
async fn example() {
    TimerFuture::new(100).await;
    info!("Done!");
}
```

**Desugars to approximately:**
```rust
fn example() -> impl Future<Output = ()> {
    enum State {
        Start,
        Waiting(TimerFuture),
        Done,
    }

    struct ExampleFuture { state: State }

    impl Future for ExampleFuture {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
            loop {
                match self.state {
                    State::Start => {
                        self.state = State::Waiting(TimerFuture::new(100));
                    }
                    State::Waiting(ref mut fut) => {
                        match Pin::new(fut).poll(cx) {
                            Poll::Ready(()) => {
                                self.state = State::Done;
                            }
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    State::Done => {
                        info!("Done!");
                        return Poll::Ready(());
                    }
                }
            }
        }
    }

    ExampleFuture { state: State::Start }
}
```

### Executors and Wakers

**Executor responsibilities:**
1. Store tasks (futures)
2. Poll tasks when they might make progress
3. Remove completed tasks

**Waker responsibilities:**
1. Signal executor when a future can make progress
2. Our dummy waker does nothing because we poll all tasks continuously

### Pin and Self-Referential Structs

`Pin<&mut Self>` ensures futures aren't moved in memory, which is important when a future contains self-referential pointers.

## Building and Running

```bash
cargo build
cargo run
```

Expected output (interleaved execution):
```
INFO  Session 6: Building a Simple Async Executor
INFO  Starting executor...
INFO  Task 1: Started
INFO  Task 2: Started
INFO  Task 1: Step 0
INFO  Task 2: Step 0
INFO  Task 1: Step 1
INFO  Task 1: Step 2
INFO  Task 2: Step 1
...
INFO  All tasks complete
```

## Limitations of This Executor

This is a teaching executor with limitations:
1. **Busy loops**: Wastes CPU polling all tasks constantly
2. **No real timers**: Uses poll counts instead of actual time
3. **No waking**: Real executors wake tasks only when needed
4. **Memory leaks**: We use `mem::transmute` for simplicity
5. **No priorities**: All tasks are equal

Real executors (like embassy-executor) solve these issues!

## Extension Challenges

1. **Add task names**: Track which task is which
2. **Count polls**: Measure how many times each task is polled
3. **Implement join**: Wait for multiple futures to complete
4. **Add select**: Complete when first future completes
5. **Real timers**: Use SysTick for actual delays instead of poll counts

## Comparison with Real Executors

**Our Simple Executor:**
- Busy loops through all tasks
- Polls every task every iteration
- Simple but inefficient

**Embassy Executor (Session 7):**
- Event-driven: tasks sleep until woken
- Interrupt-based waking
- Multiple priority levels
- Integrated with hardware peripherals

## Understanding Gained

After this session, you should understand:
- ✓ Futures are state machines
- ✓ async/await is syntactic sugar
- ✓ Executors decide when to poll
- ✓ Wakers notify executors
- ✓ Pin prevents memory issues
- ✓ Why real executors are complex!
