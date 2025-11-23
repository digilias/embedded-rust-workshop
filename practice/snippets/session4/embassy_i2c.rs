// Session 4 Snippet: Using embassy-stm32 I2C

use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::{bind_interrupts, i2c, peripherals};

// Bind I2C interrupts
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

pub fn init_i2c(p: embassy_stm32::Peripherals) -> I2c<'static, i2c::Blocking> {
    // Create I2C peripheral with blocking API
    let i2c = I2c::new_blocking(
        p.I2C1,
        p.PB8,  // SCL
        p.PB9,  // SDA
        embassy_stm32::time::Hertz(100_000),  // 100kHz
        Config::default(),
    );

    i2c
}

// Usage example:
//
// let i2c = init_i2c(p);
// let mut sensor = Lis3dh::new(i2c, 0x18);
// sensor.init().unwrap();
// loop {
//     let accel = sensor.read_accel().unwrap();
//     info!("X: {}, Y: {}, Z: {}", accel.x, accel.y, accel.z);
//     cortex_m::asm::delay(8_000_000);
// }
