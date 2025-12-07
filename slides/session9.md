---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 9: Sharing state

**Goal:** Introducing different ways to communicate between tasks

---

# Data types

* Mutex
* RefCell/Cell
* Channel
* Signal

---

# RefCell/Cell

* Holds a single reference or value
* When: you share data on  _the same_ executor (compiler will prevent you to share on multiple)
* Pros: very small overhead
* Cons: don't hold across await points!

---

# Mutex

* Two variants
  * `embassy_sync::blocking_mutex::Mutex`- blocking API should not be held across await points
  * `embassy_sync::mutex::Mutex` - async API and can be held across await points

---

# Blocking mutex

```rust
let guard = m.lock().unwrap(); // Acquires lock

let result = do_foo().await; // <- If this task yields and another task locks the same mutex: deadlock

drop(guard)
```

# Async mutex

```rust
let guard = m.lock().await; // Acquires lock

let result = do_foo().await; // <- If this task yields and another task locks the same mutex: other task yields

drop(guard)
```
  
---
# RawMutex

```rust
let m: Mutex<NoopRawMutex, T>
//        whats ^^
```

* `RawMutex` - Trait defining locking behavior
  * Use CriticalSectionMutex when data can be shared between threads and interrupts.
  * Use NoopMutex when data is only shared between tasks running on the same executor.
  * Use ThreadModeMutex when data is shared between tasks running on the same executor but you want a global singleton.
  
* `RawMutex` is used in all data types in `embassy-sync`

---

# Sync + Send

* Marker traits underlying all state sharing
* Sync: data can be shared between different _threads_
* Send: data can be moved to a different _thread_
* Threads in Rust
  * Thread mode executor
  * Interrupt executor
  * Interrupt

---

# Channel

```rust
static CHANNEL: Channel<CriticalSectionRawMutex,  u32,            10> = Channel::new();
//                           ^^ mutex type        ^^ data type    ^^ length
```

```rust
let sender: Sender<'_, CriticalSectionRawMutex, u32, 10> = CHANNEL.sender();
let receiver: Receiver<'_, CriticalSectionRawMutex, u32, 10> = CHANNEL.receiver();

// If you want dynamic dispatch
let sender: DynamicSender<'_, u32> = CHANNEL.sender().into();
let receiver: DynamicReceiver<'_, u32> = CHANNEL.sender().into();
```

---
# Signal

* Need a container of some data to share from an interrupt
* Less internal bookkeeping than Channel

---
<style scoped>
  section {
    font-size: 20px; /* Adjusts the base font size for this slide */
  }
  </style>
# Others

* Channel - A Multiple Producer Multiple Consumer (MPMC) channel. Each message is only received by a single consumer.
* PriorityChannel - A Multiple Producer Multiple Consumer (MPMC) channel. Each message is only received by a single consumer. Higher priority items are shifted to the front of the channel.
* PubSubChannel - A broadcast channel (publish-subscribe) channel. Each message is received by all consumers.
* Signal - Signalling latest value to a single consumer.
* Watch - Signalling latest value to multiple consumers.
* Mutex - Mutex for synchronizing state between asynchronous tasks.
* Pipe - Byte stream implementing embedded_io traits.
* WakerRegistration - Utility to register and wake a Waker.
* AtomicWaker - Utility to register and wake a Waker from interrupt context.
* MultiWakerRegistration - Utility registering and waking multiple Waker’s.
* LazyLock - A value which is initialized on the first access

---
# Sharing state within executor

* NoopRawMutex - pass lock to different tasks
* ThreadModeRawMutex - can store in global but only pass to tasks on same executor

---
# Sharing state with interrupts

* CriticalSectionRawMutex
* core::sync::atomic::

---
# Beware

* CriticalSectionRawMutex - disables interrupts!
* If you need to: custom critical-section implementation!


---
# Exercise

* Remove button logic
* Create 1 task for reading the accelerometer data periodically (use system timer!) and publish to a channel
* Create 1 task for consuming from a channel and log the data
