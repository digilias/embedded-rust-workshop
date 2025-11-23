// Session 4 Snippet: Using embassy-stm32 I2C with async

use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::{bind_interrupts, i2c, peripherals};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

pub fn init_i2c_async(
    p: embassy_stm32::Peripherals,
) -> I2c<'static, i2c::Async> {
    // Create I2C with async API and DMA
    let i2c = I2c::new(
        p.I2C1,
        p.PB8,  // SCL
        p.PB9,  // SDA
        Irqs,
        p.GPDMA1_CH0,  // TX DMA
        p.GPDMA1_CH1,  // RX DMA
        embassy_stm32::time::Hertz(100_000),
        Config::default(),
    );

    i2c
}

// Async usage (requires sensor driver to support embedded_hal_async::i2c::I2c):
//
// let i2c = init_i2c_async(p);
// let mut sensor = Lis3dhAsync::new(i2c, 0x18);
// sensor.init().await.unwrap();
// loop {
//     let accel = sensor.read_accel().await.unwrap();
//     info!("X: {}, Y: {}, Z: {}", accel.x, accel.y, accel.z);
//     Timer::after(Duration::from_millis(100)).await;
// }
