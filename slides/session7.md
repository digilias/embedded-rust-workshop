---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 7: Async <3 Interrupts

* **Goal:** Understand how to combine async with interrupts

---
# Control flow

* Task: triggers hardware
* Interrupt: notifies on completion

---
# Interrupts

* Point of view: a separate thread
* State shared between interrupt and app must be `Sync`

---
# Building blocks

* `Waker` - reference to a task
* `Context` - reference to _this_ task

---
# Task steps

1. Ensure that something triggers an interrupt
2. If something has happened - finished!
3. If something did not happen - make sure we get notified!

```rust
impl ButtonFuture {
    fn new() -> Self {
        // 1. Setup so that interrupt fires (once)
    }
}
impl Future for ButtonFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 2. Check if something has happened - on every poll
        // 3. If something happened - ready!
        // 4. If something did not happen - pending!
        // 5. 🤔 How to get notified?
    }
}
```

---
# Wait queues

- `WakerRegistration` - Aka. SingleEntryWaitQueue

```rust
/// Utility struct to register and wake a waker.
pub struct WakerRegistration {
    waker: Option<Waker>,
}

impl WakerRegistration {
    pub fn register(&mut self, w: &Waker) {
        ...
    }

    pub fn wake(&mut self) {
        ...
    }
}
```

* 🤔

---
# Other wait queues

- `AtomicWaker` - AKA. SingleEntryAtomicWaitQueue

```rust
/// Utility struct to register and wake a waker.
pub struct AtomicWaker {
    ...
}
impl AtomicWaker {
    pub fn register(&self, w: &Waker) {
        ...
    }
    
    pub fn wake(&self) {
        ...
    }
}
```

* 😎

---
# Tying it together

```rust
async fn dostuff() {
    loop {
        let mut f = ButtonFuture::new();
        f.await;
        info!("Interrupt fired!");
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();

#[interrupt]
fn my_irq_handler() {
    WAKER.wake();
}
```

---
# Exercise

* Create a `ButtonFuture` that sets up IRQ and does polling
* Use `AtomicWaker` to store the waker
* Modify irq handler to use waker
* Modify main to create an instance of the future and await it
