---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 5: Interrupts

* **Goal:** How to setup interrupts for the i2c sensor and reacting to it.

---
# cortex-m-rt

* Provides linker-scripts that simplifies your life
* Macros for defining the application 'entry point'
* Macros for declaring interrupt handlers
* Macros for declaring exception handlers

---
# `link.x`

* Can be auto-generated using the `memory-x` feature in HAL
* Alternative: copy linker script from cortex-m-rt and modify
* Script selection passed in `build.rs`

---
# build.rs

* build-time influence over linker parameters and compiler options
* mainly for linker scripts
* other uses
  * code-gen (metapac does this)
  * downloading blobs for linking
  
---
# `#[entry]`

* create vector table based on `#[interrupt]` and panic handlers
* copy data into RAM
* jump to application

---
# `#[interrupt]`

* Marks a function as an interrupt handler
* Function name must be named the name of the interrupt (`unsafe fn USART1()`)
* NOTE: This macro is re-exported by the PAC/HAL
  * Because most interrupts are only known by the PAC

---
# Exercise

* Create an interrupt handler for the `EXTI0` interrupt
* Use stm32-metapac and cortex-m (NVIC) to setup and react to gpio interrupt
* Support enabling/clearing interrupts in the device-driver

