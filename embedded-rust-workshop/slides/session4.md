---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 4: Using HAL for programming

*  **Goal:** Using embassy-stm32 as HAL for the device-driver application.

---
# The HAL: embassy-stm32

* Relies on metapac with a compile-time selection of PAC to use internally.
* Supports most common peripherals
* STM32 family support depends on popularity

---

# Sidenote: rust refactoring

* My language server of choice: rust-analyzer - use with most editors
* Rust & AI
  * Compiler prevents really bad hallucinations
  * Can build embassy applications
  * Good at dealing with boilerplate

---
# Exercise

* Migrate application to embassy-stm32 HAL
  * Replace I2C implementation with embassy-stm32 I2C implementation

* Add periodic sensor polling
  * Extend device-driver implementation to support reading sensor value
