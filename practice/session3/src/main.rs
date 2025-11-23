#![no_std]
#![no_main]

use defmt::*;
use stm32_metapac as pac;
use {defmt_rtt as _, panic_probe as _};

mod sensor;
mod i2c_impl;

use sensor::TemperatureSensor;
use i2c_impl::I2cBus;

const SENSOR_ADDRESS: u8 = 0x18;

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Session 3: Platform-Agnostic Drivers with embedded-hal");

    // Enable clocks for GPIOB and I2C1
    pac::RCC.ahb2enr().modify(|w| {
        w.set_gpioben(true);
    });

    pac::RCC.apb1lenr().modify(|w| {
        w.set_i2c1en(true);
    });

    // Configure I2C pins (PB8 = SCL, PB9 = SDA)
    configure_i2c_pins();

    // TODO: Create I2C bus implementation
    // let i2c = I2cBus::new(pac::I2C1, SENSOR_ADDRESS);

    // TODO: Create sensor driver using embedded-hal
    // let mut sensor = TemperatureSensor::new(i2c);

    // TODO: Initialize sensor
    // match sensor.init() {
    //     Ok(()) => info!("Sensor initialized"),
    //     Err(e) => error!("Failed to initialize sensor: {:?}", e),
    // }

    info!("TODO: Complete the I2C implementation and sensor driver");

    loop {
        // TODO: Read sensor data
        // match sensor.read_temperature() {
        //     Ok(temp) => info!("Temperature: {}.{} °C", temp / 100, temp % 100),
        //     Err(e) => error!("Failed to read temperature: {:?}", e),
        // }

        // Simple delay
        cortex_m::asm::delay(8_000_000);
    }
}

fn configure_i2c_pins() {
    use pac::gpio::vals;

    pac::GPIOB.moder().modify(|w| {
        w.set_moder(8, vals::Moder::ALTERNATE);
        w.set_moder(9, vals::Moder::ALTERNATE);
    });

    pac::GPIOB.afr(1).modify(|w| {
        w.set_afr(8 - 8, 4); // AF4 for PB8
        w.set_afr(9 - 8, 4); // AF4 for PB9
    });

    pac::GPIOB.otyper().modify(|w| {
        w.set_ot(8, vals::Ot::OPENDRAIN);
        w.set_ot(9, vals::Ot::OPENDRAIN);
    });

    pac::GPIOB.pupdr().modify(|w| {
        w.set_pupdr(8, vals::Pupdr::PULLUP);
        w.set_pupdr(9, vals::Pupdr::PULLUP);
    });
}
