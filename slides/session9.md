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
  * Blocking - may deadlock if you hold across await points!
  * Async - can be held across await points

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


# Blocking Mutex

```rust
Mutex<M: RawMutex, T>
```

* `RawMutex` - trait defining the acquire/release semantics
* `NoopRawMutex` - No guard

---

# Channel

---

# Signal

---


# Sharing state within executor

---

# Sharing state with interrupts

---
# Sharing global state

---
# Beware
