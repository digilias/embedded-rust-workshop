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

## Session 8

* https://github.com/embassy-rs/embassy/blob/main/examples/stm32h5/src/bin/blinky.rs#L10-L11

* https://docs.rs/lis3dh-async/0.9.3/lis3dh_async/trait.Lis3dhCore.html#tymethod.read_register
* https://docs.rs/lis3dh-async/0.9.3/lis3dh_async/enum.Register.html

* https://docs.embassy.dev/embassy-stm32/git/stm32h563zi/exti/struct.ExtiInput.html

## Session 9

* https://docs.embassy.dev/embassy-sync/git/default/channel/struct.Channel.html
* https://docs.embassy.dev/embassy-time/git/default/struct.Timer.html
* https://docs.rs/lis3dh-async/latest/lis3dh_async/struct.Lis3dh.html#method.accel_raw

```rust

  static CHANNEL: Channel<ThreadModeRawMutex, Sample, 10> = Channel::new();
  #[embassy_executor::task]
  async fn producer(mut xl: Lis3dh<Lis3dhI2C<I2c<'static, Async, i2c::Master>>>, sender: Sender<'static, ThreadModeRawMutex, Sample, 10>) {}

  #[embassy_executor::task]
  async fn consumer(receiver: Receiver<'static, ThreadModeRawMutex, Sample, 10>) {}
```

## Session 10

* https://docs.rs/lis3dh-async/latest/lis3dh_async/struct.Lis3dh.html#method.configure_interrupt_pin
* https://docs.rs/lis3dh-async/latest/lis3dh_async/struct.Lis3dh.html#method.configure_irq_src
* https://docs.embassy.dev/embassy-stm32/git/stm32h563zi/exti/struct.ExtiInput.html (PA5)

  ```rust
    let dr = DataRate::Hz_1;
    unwrap!(xl.set_datarate(dr).await);

    unwrap!(xl.configure_irq_src(
        Interrupt1,
        InterruptMode::Position,
        InterruptConfig::high_and_low(),
    ).await);

    // Raise pin state if interrupt 1 is raised and there is movement
    unwrap!(xl.configure_interrupt_pin(IrqPin1Config {
        zyxda_en: true,
        ..IrqPin1Config::default()
    }).await);
  ```
## Session 11 

* https://crates.io/crates/panic-persist
* https://crates.io/crates/panic-halt
* https://crates.io/crates/panic-reset
* https://crates.io/crates/panic-probe

## Session 12

* https://crates.io/crates/bindgen
* See ../libs/lowpass_filter/include/lowpass_filter.h
* See ../libs/lowpass_filter/Cargo.toml 
* See ../libs/lowpass_filter/build.rs
* See ../libs/lowpass_filter/src/lib.rs

## Session 13

* https://docs.embassy.dev/embassy-net/git/default/struct.Config.html
* https://docs.embassy.dev/embassy-net/git/default/struct.Stack.html
* https://docs.embassy.dev/embassy-net/git/default/tcp/struct.TcpSocket.html

* Client
  ```rust
  use embassy_net::tcp::TcpSocket;

  let mut tx_buffer = [0u8; 2048];
  let mut rx_buffer = [0u8; 2048];
  let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

  // Connect to server
  socket.connect(remote_endpoint).await?;

  // Write data
  socket.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
  ```
