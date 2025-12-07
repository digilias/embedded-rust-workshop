# workshop

## Session 2

* https://docs.embassy.dev/embassy-stm32/git/stm32h563zi/index.html
* https://docs.embassy.dev/embassy-stm32/git/stm32h563zi/i2c/struct.I2c.html#method.new_blocking
* https://docs.embassy.dev/embassy-stm32/git/stm32h563zi/i2c/struct.I2c.html#method.blocking_read

* const ADDRESS: u8 = 0x18;
* const WHOAMI: u8 = 0x0F;
* I2C: I2C1
* SCL: PB8
* SDA: PB9

## Session 3

* https://crates.io/crates/device-driver
* https://docs.rs/device-driver/1.0.7/device_driver/trait.RegisterInterface.html

```rust
device_driver::create_device!(
    device_name: Lis3dh,
    dsl: {
        ...
    }
);
```

## Session 4

* https://crates.io/crates/embassy-time
* https://docs.embassy.dev/embassy-time/git/default/fn.block_for.html

## Session 5

* https://crates.io/crates/cortex-m-rt
* https://docs.rs/cortex-m/0.7.7/cortex_m/peripheral/struct.NVIC.html
* https://docs.embassy.dev/embassy-stm32/git/stm32h563zi/gpio/index.html
* PIN: 13
* PORT: 2

## Session 6

* https://doc.rust-lang.org/core/future/trait.Future.html
* https://doc.rust-lang.org/core/task/index.html

## Session 7

* https://doc.rust-lang.org/core/future/trait.Future.html#tymethod.poll
* https://docs.embassy.dev/embassy-sync/git/default/waitqueue/struct.AtomicWaker.html
