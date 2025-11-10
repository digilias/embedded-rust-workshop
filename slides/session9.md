---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 9: Sharing state

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

// If you are ok with dynamic dispatch
let sender: DynamicSender<'_, u32> = CHANNEL.sender().into();
let receiver: DynamicReceiver<'_, u32> = CHANNEL.sender().into();
```

---

# Signal

* Need a container of some data to share from an interrupt
* Less internal bookkeeping than channel
* 

---

# Sharing state within executor

---

# Sharing state with interrupts

---
# Sharing global state

---
# Beware
