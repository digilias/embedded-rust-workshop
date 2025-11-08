#![no_std]
#![no_main]

use defmt::*;
use pac::i2c::vals;
use stm32_metapac as pac;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x18;
const WHOAMI: u8 = 0x0F;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Enable clocks for GPIOB and I2C1
    // Using I2C1 on PB8 (SCL) and PB9 (SDA) as an example
    pac::RCC.ahb2enr().modify(|w| {
        w.set_gpioben(true); // Enable GPIOB clock
    });

    pac::RCC.apb1lenr().modify(|w| {
        w.set_i2c1en(true); // Enable I2C1 clock
    });

    // Configure PB8 and PB9 as alternate function for I2C1
    // PB8 = I2C1_SCL (AF4)
    // PB9 = I2C1_SDA (AF4)
    p.GPIOB.moder().modify(|w| {
        w.set_moder(8, vals::Moder::ALTERNATE); // PB8 as alternate function
        w.set_moder(9, vals::Moder::ALTERNATE); // PB9 as alternate function
    });

    p.GPIOB.afr(0).modify(|w| {
        w.set_afr(8, 0x04); // AF4 for PB8
        w.set_afr(9, 0x04); // AF4 for PB9
    });

    // Configure open-drain output for I2C pins
    p.GPIOB.otyper().modify(|w| {
        w.set_ot(8, vals::Ot::OPENDRAIN);
        w.set_ot(9, vals::Ot::OPENDRAIN);
    });

    // Configure pull-up resistors (internal pull-ups for I2C)
    p.GPIOB.pupdr().modify(|w| {
        w.set_pupdr(8, vals::Pupdr::PULLUP);
        w.set_pupdr(9, vals::Pupdr::PULLUP);
    });

    // Configure I2C1
    let i2c1 = p.I2C1;

    // Disable I2C1 before configuration
    i2c1.cr1().modify(|w| w.set_pe(false));

    // Configure timing for I2C
    // This is for 100kHz Standard Mode with 64MHz I2C clock
    // You may need to adjust based on your clock configuration
    // PRESC = 15, SCLL = 0x13, SCLH = 0xF, SDADEL = 0x2, SCLDEL = 0x4
    i2c1.timingr().write(|w| {
        w.set_presc(15);
        w.set_scll(0x13);
        w.set_sclh(0x0F);
        w.set_sdadel(0x02);
        w.set_scldel(0x04);
    });

    // Enable I2C1
    i2c1.cr1().modify(|w| w.set_pe(true));

    // Small delay to let I2C stabilize
    cortex_m::asm::delay(1000);

    // Read WHO_AM_I register
    let who_am_i = read_register(&i2c1, SENSOR_I2C_ADDR, WHO_AM_I_REG);

    // Store result in a variable (in real application, you'd use RTT or UART to print)
    let _sensor_id = match who_am_i {
        Ok(id) => id,
        Err(_) => 0xFF, // Error value
    };

    // Main loop
    loop {
        cortex_m::asm::wfi();
    }
}

fn read_register(i2c: &pac::i2c::I2c, addr: u8, reg: u8) -> Result<u8, I2cError> {
    // Wait until I2C is not busy
    wait_busy(i2c)?;

    // Configure for 1-byte write (register address)
    i2c.cr2().modify(|w| {
        w.set_sadd((addr as u16) << 1); // 7-bit address mode
        w.set_nbytes(1); // 1 byte to write
        w.set_rd_wrn(vals::RdWrn::WRITE); // Write mode
        w.set_autoend(false); // No auto end
        w.set_start(true); // Generate start
    });

    // Wait for TX buffer to be empty and send register address
    wait_txis(i2c)?;
    i2c.txdr().write(|w| w.set_txdata(reg));

    // Wait for transfer complete
    wait_tc(i2c)?;

    // Configure for 1-byte read
    i2c.cr2().modify(|w| {
        w.set_sadd((addr as u16) << 1); // 7-bit address mode
        w.set_nbytes(1); // 1 byte to read
        w.set_rd_wrn(vals::RdWrn::READ); // Read mode
        w.set_autoend(true); // Auto end after 1 byte
        w.set_start(true); // Generate repeated start
    });

    // Wait for data to be received
    wait_rxne(i2c)?;

    // Read the received byte
    let data = i2c.rxdr().read().rxdata();

    // Wait for stop condition
    wait_stop(i2c)?;

    // Clear stop flag
    i2c.icr().write(|w| w.set_stopcf(true));

    defmt::info!("Data: {:x}", data);
    loop {}
}

// Helper functions for I2C status checking
fn wait_busy(i2c: &pac::i2c::I2c) -> Result<(), I2cError> {
    let mut timeout = 100000;
    while i2c.isr().read().busy() {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
    }
    Ok(())
}

fn wait_txis(i2c: &pac::i2c::I2c) -> Result<(), I2cError> {
    let mut timeout = 100000;
    loop {
        let isr = i2c.isr().read();
        if isr.txis() {
            return Ok(());
        }
        if isr.nackf() {
            return Err(I2cError::Nack);
        }
        timeout -= 1;
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
    }
}

fn wait_tc(i2c: &pac::i2c::I2c) -> Result<(), I2cError> {
    let mut timeout = 100000;
    loop {
        let isr = i2c.isr().read();
        if isr.tc() {
            return Ok(());
        }
        if isr.nackf() {
            return Err(I2cError::Nack);
        }
        timeout -= 1;
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
    }
}

fn wait_rxne(i2c: &pac::i2c::I2c) -> Result<(), I2cError> {
    let mut timeout = 100000;
    loop {
        let isr = i2c.isr().read();
        if isr.rxne() {
            return Ok(());
        }
        if isr.nackf() {
            return Err(I2cError::Nack);
        }
        timeout -= 1;
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
    }
}

fn wait_stop(i2c: &pac::i2c::I2c) -> Result<(), I2cError> {
    let mut timeout = 100000;
    while !i2c.isr().read().stopf() {
        timeout -= 1;
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
    }
    Ok(())
}

#[derive(Debug)]
enum I2cError {
    Timeout,
    Nack,
}
