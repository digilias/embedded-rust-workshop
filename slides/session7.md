---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 7: Embassy executor

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
# Embassy's executor

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
# Executor and tasks

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

---
