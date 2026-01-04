---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 10: Executor and preemption

* **Goal**: Understanding task scheduling and preemption

--- 
# The problem

```rust
static SHARED: Mutex<ThreadModeRawMutex, Cell<u32>> = Mutex::new(Cell::new(0));

#[embassy_executor::task]
async fn high_prio() {
    loop {
        Timer::after(Duration::from_millis(500)).await;
        SHARED.lock(|d| d.set(d.get() + 1));
    }
}

#[embassy_executor::task]
async fn low_prio() {
    loop {
        Timer::after(Duration::from_secs(1)).await;
        let val = SHARED.lock(|d| d.get());
        defmt::info!("Val: {}", val);
    }
}

#[embassy_executor::main]
async fn main(s: Spawner) {
    s.spawn(high_prio().unwrap());
    s.spawn(low_prio().unwrap());
}
```

--- 
# Task scheduling priority

```toml
embassy-executor = { features = ["scheduler-priority"] } # Or: scheduler-deadline
```

```rust
#[embassy_executor::main]
async fn main(s: Spawner) {
    let t = high_prio().unwrap();
    t.metadata().set_priority(0);
    s.spawn(t);

    let t = low_prio().unwrap();
    t.metadata().set_priority(1);
    s.spawn(t);
}
```

--- 
# Task scheduling priority

* Same executor: on single core systems tasks on the same executor can share global state!
* Still: interrupts still need to use critical section to share data with tasks.
* Higher priority still need to wait for lower priority to block

--- 
# Dealing with preemption

```rust
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: InterruptExecutor = InterruptExecutor::new();

interrupt::EGU1_SWI1.set_priority(Priority::P0);
interrupt::EGU2_SWI2.set_priority(Priority::P6);

EXECUTOR_HIGH.start(interrupt::EGU1_SWI1).spawn(high_prio().unwrap());
EXECUTOR_LOW.start(interrupt::EGU1_SWI1).spawn(low_prio().unwrap());
```

---
# Sharing state

* Tasks in different executors has to use `CriticalSectionRawMutex`.
* Enforced by compiler :tada:

---
# Exercises

* Setup the lis3dh accelerometer to signal reading via IRQ
* Instead of sampling periodically - await IRQ
