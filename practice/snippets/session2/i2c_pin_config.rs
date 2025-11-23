// Session 2 Snippet: I2C Pin Configuration
// Configure PB8 (SCL) and PB9 (SDA) for I2C1

use stm32_metapac as pac;
use pac::gpio::vals;

pub fn configure_i2c_pins() {
    // Enable GPIOB clock
    pac::RCC.ahb2enr().modify(|w| {
        w.set_gpioben(true);
    });

    // Configure PB8 and PB9 as alternate function
    pac::GPIOB.moder().modify(|w| {
        w.set_moder(8, vals::Moder::ALTERNATE); // PB8 = SCL
        w.set_moder(9, vals::Moder::ALTERNATE); // PB9 = SDA
    });

    // Set alternate function to AF4 (I2C1)
    pac::GPIOB.afr(1).modify(|w| {
        w.set_afr(8 - 8, 4); // PB8 -> AF4
        w.set_afr(9 - 8, 4); // PB9 -> AF4
    });

    // Configure as open-drain (required for I2C)
    pac::GPIOB.otyper().modify(|w| {
        w.set_ot(8, vals::Ot::OPENDRAIN);
        w.set_ot(9, vals::Ot::OPENDRAIN);
    });

    // Enable internal pull-ups
    pac::GPIOB.pupdr().modify(|w| {
        w.set_pupdr(8, vals::Pupdr::PULLUP);
        w.set_pupdr(9, vals::Pupdr::PULLUP);
    });
}
