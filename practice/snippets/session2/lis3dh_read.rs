// Session 2 Snippet: Reading WHO_AM_I from LIS3DH
// Complete example of I2C read transaction

use stm32_metapac as pac;
use pac::i2c::vals;

const LIS3DH_ADDR: u8 = 0x18;
const WHO_AM_I: u8 = 0x0F;

#[derive(Debug)]
pub enum I2cError {
    Timeout,
    Nack,
}

pub fn read_who_am_i() -> Result<u8, I2cError> {
    let i2c = pac::I2C1;

    // Wait until bus is not busy
    wait_not_busy(&i2c)?;

    // Configure for 1-byte write (register address)
    i2c.cr2().modify(|w| {
        w.set_sadd((LIS3DH_ADDR as u16) << 1); // 7-bit address in bits [7:1]
        w.set_nbytes(1);                        // Send 1 byte
        w.set_rd_wrn(vals::RdWrn::WRITE);      // Write mode
        w.set_autoend(false);                   // Don't auto-end
        w.set_start(true);                      // Generate START
    });

    // Wait for TX ready and send register address
    wait_txis(&i2c)?;
    i2c.txdr().write(|w| w.set_txdata(WHO_AM_I));

    // Wait for transfer complete
    wait_tc(&i2c)?;

    // Configure for 1-byte read
    i2c.cr2().modify(|w| {
        w.set_sadd((LIS3DH_ADDR as u16) << 1);
        w.set_nbytes(1);                        // Read 1 byte
        w.set_rd_wrn(vals::RdWrn::READ);       // Read mode
        w.set_autoend(true);                    // Auto STOP after read
        w.set_start(true);                      // Generate repeated START
    });

    // Wait for RX ready
    wait_rxne(&i2c)?;

    // Read the data
    let data = i2c.rxdr().read().rxdata();

    // Wait for STOP
    wait_stop(&i2c)?;

    // Clear STOP flag
    i2c.icr().write(|w| w.set_stopcf(true));

    Ok(data)
}

// Helper functions for checking I2C status

fn wait_not_busy(i2c: &pac::i2c::I2c) -> Result<(), I2cError> {
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
