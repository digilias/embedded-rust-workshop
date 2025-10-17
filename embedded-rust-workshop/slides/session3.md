---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 3: Platform agnostic drivers


* **Goal:** Using embedded-hal + device-driver to create generic device drivers.

---

# Rust Embedded Working Group
* **Coordinates with Rust Language team**
* **Maintains a few crates (libraries) that "everyone" uses**
  - **cortex-m/cortex-ar/riscv** - low level access to peripherals
  - **cortex-m-rt** - minimal runtime for cortex-m
  - **embedded-hal(-async)** - traits for SPI, I2C, GPIO ++
  - **heapless** - collections without allocator
* **Weekly meetings in the matrix chat Tuesdays, 8pm CET/CEST**

---
# embedded-hal

* Traits for 

* GPIO
* SPI
* I2C

---
# Example: i2c

```rust
use embedded_hal::i2c::{I2c, Error};

const ADDR: u8 = 0x15;
pub struct TemperatureSensorDriver<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> TemperatureSensorDriver<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_temperature(&mut self) -> Result<u8, I2C::Error> {
        let mut temp = [0];
        self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp)?;
        Ok(temp[0])
    }
}
```

---

# embedded-hal-async

* Same as `embedded-hal`, but async!

---
# Example: i2c


```rust
    pub async fn read_temperature(&mut self) -> Result<u8, I2C::Error> {
        let mut temp = [0];
        self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp).await?;
        Ok(temp[0])
    }
```

---

# bus sharing

* embedded-hal-bus
* embassy-embedded-hal

---
# Example: bus sharing

```rust
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

static I2C_BUS: StaticCell<Mutex<NoopRawMutex, Twim<TWISPI0>>> = StaticCell::new();

let config = twim::Config::default();
let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_03, p.P0_04, config);
let i2c_bus = Mutex::new(i2c);
let i2c_bus = I2C_BUS.init(i2c_bus);

// Device 1, using embedded-hal-async compatible driver for QMC5883L compass
let i2c_dev1 = I2cDevice::new(i2c_bus);
let compass = QMC5883L::new(i2c_dev1).await.unwrap();

// Device 2, using embedded-hal-async compatible driver for Mpu6050 accelerometer
let i2c_dev2 = I2cDevice::new(i2c_bus);
let mpu = Mpu6050::new(i2c_dev2);
```
---

# embedded-io

* Input/Output traits
* std::io but for embedded
* Read, BufRead, Write, BufRead
* Useful for: uart, tcp/ip

---

# embedded-io-async

* Same, but async!

---

# device-driver

* Saves you time to write drivers
* Use a DSL or JSON to define registers and values

---

# Exercise

* Implement a driver using the `device-driver` for the rotary encoder
* Example skeleton can be found in the session3 folder

