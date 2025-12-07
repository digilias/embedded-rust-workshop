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
# SimpleExecutor vs Embassy Executor

* Task macros to "build" task list
* Sleep/wakeup mechanism
* Priorities and deadlines
* Metadata
* System timer
* ...

---
# Integration with HAL - system timer

* `embassy-time` - Timer API
  * `Instant` - current time
  * `Duration` - time duration
  * `Timer` - for delaying
* `embassy-time-driver` - integration with `embassy-executor`
  * Time driver creates "alarms" that wake the executor

---
# Integration with HAL - interrupts

* How do we prevent conflicting interrupt handling?

* ```rust
  bind_interrupts!(pub struct Irqs {
      I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
      I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
      EXTI13 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI13>;
  });
  ```

* `Irqs` is passed to the HAL when necessary to document the contract
* You cannot both use the `interrupt` macro and `bind_interrupts` with the same interrupt!

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
# Code size 

* Executor::run - 316B
* Executor::run (feature = turbowakers) - 286B
* Waker - 214B
* Total - 500-530B
  
---
# Speed

* Cycles per poll: 78-91 

---
# RAM
* Task stack is perfectly sized
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
# Exercise - time to simplify!

1. Refactor and replace simple loop executor with embassy main
2. Try running your program again (it should work without modifications!)
3. Replace our own lis3dh-driver with the one from lis3dh-async 
4. Try running your program again
5. Replace interrupt code with `ExtiInput` from `embassy-stm32`
