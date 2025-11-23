// Session 2 Snippet: I2C Peripheral Initialization
// Initialize I2C1 for 100kHz operation

use stm32_metapac as pac;

pub fn init_i2c() {
    // Enable I2C1 clock
    pac::RCC.apb1lenr().modify(|w| {
        w.set_i2c1en(true);
    });

    let i2c = pac::I2C1;

    // Disable I2C before configuration
    i2c.cr1().modify(|w| w.set_pe(false));

    // Configure timing for 100kHz with 64MHz I2C clock
    // These values are for STM32H5 with specific clock configuration
    // PRESC=15, SCLL=0x13, SCLH=0x0F, SDADEL=0x02, SCLDEL=0x04
    i2c.timingr().write(|w| {
        w.set_presc(15);     // Prescaler
        w.set_scll(0x13);    // SCL low period
        w.set_sclh(0x0F);    // SCL high period
        w.set_sdadel(0x02);  // SDA delay
        w.set_scldel(0x04);  // SCL delay
    });

    // Enable I2C peripheral
    i2c.cr1().modify(|w| w.set_pe(true));

    // Small delay to let peripheral stabilize
    cortex_m::asm::delay(1000);
}
