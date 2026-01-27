---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 2: PAC and HAL

* **Goal:** Introducing using peripherals with the Peripheral Access Crate (PAC) concept and using that to work with registers.

---
![bg 90%](rust_layers.svg)

---
# PACs in the wild

* [stm32-rs](https://github.com/stm32-rs)
  - Oldest
  - Separate crates for each chip family
  - Level of features supported vary greatly
  - Using svd2rust
* [stm32-metapac](https://crates.io/crates/stm32-metapac)
  - Supports all STM32 variants
  - Chip variant selected using feature flag
  - Used by the embassy-stm32 HAL
  - Using chiptool
   

---
# Source of truth

* [stm32-data](https://github.com/embassy-rs/stm32-data)
* Scripts for downloading metadata from vendors
* Rust build scripts for generating Rust code from vendor sources
* Applies transforms to treat arrays, fields in a specific way

---
# Transformers

```toml
  - !MakeFieldArray
    fieldsets: IBIDR
    from: ^(IBIDB)\d$
    to: $1

  - !MergeFieldsets
    from: ^(DEVR)[1-4]$
    to: $1

  - !MakeRegisterArray
    blocks: I3C
    from: ^(DEVR)[1-4]$
    to: $1

  - !RenameFields
    fieldset: ^MAX[RW]LR$
    from: ^M[RW]L$
    to: ML
```


---

# Output

* Generated data -> [stm32-data-generated](https://github.com/embassy-rs/stm32-data-generated)
* A feature support table 
* JSON description of chip ([example](https://github.com/embassy-rs/stm32-data-generated/blob/main/data/chips/STM32H563ZI.json))

---

# 🤯

* "Do I need to understand all this?!"
* No
* But, these are (so far) maintained by community
* So if you need support for some peripheral, this is where you go to add it.

---

# The "meatpack"

* [stm32-metapac](https://crates.io/crates/stm32-metapac)
* In a nutshell: An advanced Rust build-time dependency (build.rs) that reads JSON and generates Rust code from it.

---

# Cargo.toml

```toml
stm32-metapac = { version = "18", features = ["stm32h563zi"] }
```

---
# Where's the code?

* `find target -type f -name pac.rs`
* Works with IDE completion

---
# Blinky with PAC (part 1)

```rust
#![no_std]
#![no_main]

use pac::gpio::vals;
use {defmt_rtt as _, panic_probe as _, stm32_metapac as pac};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Enable GPIO clock
    let rcc = pac::RCC;
    rcc.ahb2enr().modify(|w| {
        w.set_gpioben(true);
    });

    rcc.ahb2rstr().modify(|w| {
        w.set_gpiobrst(true);
        w.set_gpiobrst(false);
    });
```

---
# Blinky with PAC (part 2)

```rust
    // Setup LED
    let gpiob = pac::GPIOB;
    const LED_PIN: usize = 00;
    gpiob.pupdr().modify(|w| w.set_pupdr(LED_PIN, vals::Pupdr::FLOATING));
    gpiob.otyper().modify(|w| w.set_ot(LED_PIN, vals::Ot::PUSH_PULL));
    gpiob.moder().modify(|w| w.set_moder(LED_PIN, vals::Moder::OUTPUT));

    let mut i = 0;
    loop {
        if i % 2 == 0 {
            gpiob.bsrr().write(|w| w.set_br(LED_PIN, true));
        } else {
            gpiob.bsrr().write(|w| w.set_bs(LED_PIN, true));
        }
        for i in 0..1000000 {
            cortex_m::asm::nop();
        }
        i += 1;
    }
}
```

---
# Blinky with HAL

```rust
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(500).await;

        info!("low");
        led.set_low();
        Timer::after_millis(500).await;
    }
}
```

---
# Peripherals

* `Peripherals` struct provided by the HAL
* Singleton that you can pass around or borrow from

---
# Blinky with HAL and borrowing

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = embassy_stm32::init(Default::default());
    {
        let mut led = Output::new(&mut p.PB0, Level::High, Speed::Low);
        let mut i = 0;
        while i < 10 {
            info!("high");
            led.set_high();
            Timer::after_millis(500).await;

            info!("low");
            led.set_low();
            Timer::after_millis(500).await;
            i += 1;
        }
    }
    // `led` out of scope -> you can use it again
}
```

---

# Exercise

* Start with the workshop project
* Create an instance of the embassy stm32 HAL
* Create an instance of i2c
* Read the who am i register
* Run on devkit
