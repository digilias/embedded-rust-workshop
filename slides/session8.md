---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 8: Embassy executor

* **Goal:** Understanding the embassy executor

--- 
# Embassy

* **Collection of libraries**
  - Use what you want
  - Even works without async
* **Executor:** "Runtime" for async tasks
* **HAL:** For nRF, STM32, RP, ESP32, NXP, MSPM, IMXRT
* **Utilities:** async channels, mutex, time keeping
* **Networking:** drivers integrated with smoltcp (Rust TCP/IP stack)
* **Bluetooth LE:** HCI host implementation
* **Bootloader**
* **USB**

--- 
<style scoped>
  section {
    font-size: 24px; /* Adjusts the base font size for this slide */
  }
  </style>
<!-- _class: green-accent -->
# Example

```rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);
    loop {
        button.wait_for_any_edge().await;
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
```

---
<style scoped>
  section {
    font-size: 24px; /* Adjusts the base font size for this slide */
  }
  </style>
<!-- _class: green-accent -->
# Tasks

```rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let led = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);
    spawner.spawn(my_task(button, led).unwrap());
}

#[embassy_executor::task]
async fn my_task(mut button: ExtiInput, mut led: Output) {
    loop {
        button.wait_for_any_edge().await;
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
```

---
  
# Expanded task macro

```rust
static MY_TASK: Task<F> = Task::new();
fn my_task(mut button: ExtiInput, mut led: Output) -> SpawnToken {
    MY_TASK.init(my_task_inner())
}

async fn my_task_inner(mut button: ExtiInput, mut led: Output) {
    loop {
        // ...
    }
}
```

* Problem: using generics in statics
* Solution: clever use of const and macros to figure out the type

---
# What alternatives are there?

* RTIC
* smol
* ... your own?

---
# RTIC

```
#[rtic::app(device = lm3s6965)]
mod app {
    // ...

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        rtic::pend(Interrupt::UART0); // equivalent to NVIC::pend

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = UART0, local = [times: u32 = 0])]
    fn uart0(cx: uart0::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        info!(
            "UART0 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        );
    }
}
```

---
# Metrics

* Code size
  * Executor::run - 316B
  * Executor::run (feature = turbowakers) - 286B
  * Waker - 214B
  * Total - 500-530B
  
* Execution speed
  * Cycles per poll: 78-91 

* RAM
  * Task stack is perfectly sizedPerfectly sized stack
  * Per-task executor state (13 bytes):
    * Task state: 1 byte
    * Executor ptr: 4 bytes
    * Run queue item: 4 bytes
    * Time queue item (optional): 4 bytes
    * Metadata: 0+ bytes

---
# Philosophy of embassy

* Reduce unnecessary abstractions
* Consistency across HALs
* Pick and choose your crates
* Don't solve problems users don't have
* Important metrics: code size, execution time and ram usage

---

# Exercise

* Remove simple loop executor
* Use embassy executor for main
* 
* Add async delays and timeouts
