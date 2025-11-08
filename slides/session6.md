---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 6: Async Fundamentals

* **Goal:** Understanding how async works.

---
# Concurrency models

* Threads
* Coroutines
* Generators

---
<style scoped>
  section {
    align-content: start;
  }
  </style>
<!-- _class: code-slide green-accent -->
# What is async?

```rust
async fn do_stuff() -> bool { return true }
```
---
<style scoped>
  section {
    align-content: start;
  }
  </style>
<!-- _class: code-slide green-accent -->
# What is async?

```rust
async fn do_stuff() -> bool { return true }
```

**Is the same as**

```rust
fn do_stuff() -> impl Future<Output = bool> { async { true } }
```

---
<style scoped>
  section {
    align-content: start;
  }
  </style>
<!-- _class: code-slide green-accent -->
# What is async?

```rust
async fn do_stuff() -> bool { return true }
```

**Is the same as**

```rust
fn do_stuff() -> impl Future<Output = bool> { async { true } }
```

**Is the same as**

```rust
struct DoStuffFuture;
impl Future for DoStuffFuture {
    type Output = bool;
    fn poll(..., cx: &mut Context<'_>) -> Poll<bool> { Poll::Ready(true) }
}
fn do_stuff() -> DoStuffFuture
```
---
# Futures

* Introduced in 2018 edition
* Fundamentally changed the concurrency story in Rust
* "Asynchronous computation"

---
# Future trait


```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

* Polling - checking if the computation can continue
* Yielding - returning `Poll::Pending` when it can't make progress

---

# Sidenote: Pin

* What if the future was moved between polls?
  * The `poll()` implementation could reference variables in its own type
  * BOOM!
* Pin: enforce that it cannot be moved!

---
<!-- _class: green-accent -->
# The executor

* Maintains list of tasks
* Decides which task to poll
* Polls the future with the context of that future
* If no progress -> move on to next

![bg right 80%](embassy_executor.png)


---
# Send and Sync marker traits

* Send: type can be handed to another thread
* Sync: type can be used in two threads

* Impacts how you share data between _tasks_ running on the same or different executor

---
# Exercise

* Create a simple busy-loop executor
