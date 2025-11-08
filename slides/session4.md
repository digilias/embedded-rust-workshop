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
# HAL philosophy

* Cover 80-90% of use cases
* The 10% specials can be solved with PAC
* Don't: contributing changes that just expose internals of the HAL
* Do: create mechanisms that are useful for other use cases

---
# The HAL: embassy-stm32

* Relies on metapac with a compile-time selection of PAC to use internally.
* Supports most common peripherals
* STM32 family support depends on popularity
* Register blocks grouped into versions
  * Allows reusing peripheral code for one chip with another
---

# Hardware support

* Depends on chip family
* DMA, GPIO, TIMER, ADC, CAN, CORDIC, CRC, CRYP, DAC, DCMI, DSIHOST, DTS, ETH, EXTI, FLASH, FMC, HASH, HRTIM, HSEM, HSPI, I2C, I2S, IPCC, LPTIM, OPAMP, OSPI, QSPI, RNG, RTC, SDCMMC, SPI, TSC, USB, WDG, XSPI

---

# Time driver

* Built-in time-keeping
* `embassy-time` - API for time, delays, duration etc.
* `embassy-time-driver` - C ABI for integrating timer with executor
* `embassy-executor` - Use `time` feature to enable time driver integration.
* `embassy-stm32` - Use `time` feature to enable time driver integration.

---

# Error handling

* Error defined in peripheral module
* Converted to embedded-hal error types if relevant (i2c, spi, gpio etc)

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
