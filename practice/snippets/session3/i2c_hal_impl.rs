// Session 3 Snippet: embedded-hal I2c Trait Implementation

use embedded_hal::i2c::{Error, ErrorKind, ErrorType, I2c};
use stm32_metapac as pac;

pub struct I2cBus {
    i2c: pac::i2c::I2c,
}

#[derive(Debug)]
pub struct I2cError {
    kind: ErrorKind,
}

impl Error for I2cError {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl I2cBus {
    pub fn new(i2c: pac::i2c::I2c) -> Self {
        Self { i2c }
    }

    // Helper: wait for bus not busy
    fn wait_not_busy(&self) -> Result<(), I2cError> {
        let mut timeout = 100000;
        while self.i2c.isr().read().busy() {
            timeout -= 1;
            if timeout == 0 {
                return Err(I2cError {
                    kind: ErrorKind::Bus,
                });
            }
        }
        Ok(())
    }

    // Add more helper methods as needed (wait_txis, wait_tc, wait_rxne, etc.)
}

impl ErrorType for I2cBus {
    type Error = I2cError;
}

impl I2c for I2cBus {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.wait_not_busy()?;

        // Configure for read
        self.i2c.cr2().modify(|w| {
            w.set_sadd((address as u16) << 1);
            w.set_nbytes(buffer.len() as u8);
            w.set_rd_wrn(pac::i2c::vals::RdWrn::READ);
            w.set_autoend(true);
            w.set_start(true);
        });

        // Read each byte
        for byte in buffer.iter_mut() {
            // Wait for RXNE
            let mut timeout = 100000;
            loop {
                let isr = self.i2c.isr().read();
                if isr.rxne() {
                    break;
                }
                if isr.nackf() {
                    return Err(I2cError {
                        kind: ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    });
                }
                timeout -= 1;
                if timeout == 0 {
                    return Err(I2cError {
                        kind: ErrorKind::Bus,
                    });
                }
            }

            *byte = self.i2c.rxdr().read().rxdata();
        }

        Ok(())
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.wait_not_busy()?;

        // Configure for write
        self.i2c.cr2().modify(|w| {
            w.set_sadd((address as u16) << 1);
            w.set_nbytes(bytes.len() as u8);
            w.set_rd_wrn(pac::i2c::vals::RdWrn::WRITE);
            w.set_autoend(true);
            w.set_start(true);
        });

        // Write each byte
        for &byte in bytes {
            // Wait for TXIS
            let mut timeout = 100000;
            loop {
                let isr = self.i2c.isr().read();
                if isr.txis() {
                    break;
                }
                if isr.nackf() {
                    return Err(I2cError {
                        kind: ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Data),
                    });
                }
                timeout -= 1;
                if timeout == 0 {
                    return Err(I2cError {
                        kind: ErrorKind::Bus,
                    });
                }
            }

            self.i2c.txdr().write(|w| w.set_txdata(byte));
        }

        Ok(())
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.wait_not_busy()?;

        // Write phase (no autoend for restart)
        self.i2c.cr2().modify(|w| {
            w.set_sadd((address as u16) << 1);
            w.set_nbytes(bytes.len() as u8);
            w.set_rd_wrn(pac::i2c::vals::RdWrn::WRITE);
            w.set_autoend(false);  // No STOP yet
            w.set_start(true);
        });

        for &byte in bytes {
            let mut timeout = 100000;
            loop {
                let isr = self.i2c.isr().read();
                if isr.txis() {
                    break;
                }
                if isr.nackf() {
                    return Err(I2cError {
                        kind: ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Data),
                    });
                }
                timeout -= 1;
                if timeout == 0 {
                    return Err(I2cError {
                        kind: ErrorKind::Bus,
                    });
                }
            }
            self.i2c.txdr().write(|w| w.set_txdata(byte));
        }

        // Wait for TC
        let mut timeout = 100000;
        while !self.i2c.isr().read().tc() {
            timeout -= 1;
            if timeout == 0 {
                return Err(I2cError {
                    kind: ErrorKind::Bus,
                });
            }
        }

        // Read phase (with autoend)
        self.i2c.cr2().modify(|w| {
            w.set_sadd((address as u16) << 1);
            w.set_nbytes(buffer.len() as u8);
            w.set_rd_wrn(pac::i2c::vals::RdWrn::READ);
            w.set_autoend(true);
            w.set_start(true);  // Repeated START
        });

        for byte in buffer.iter_mut() {
            let mut timeout = 100000;
            loop {
                let isr = self.i2c.isr().read();
                if isr.rxne() {
                    break;
                }
                if isr.nackf() {
                    return Err(I2cError {
                        kind: ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Data),
                    });
                }
                timeout -= 1;
                if timeout == 0 {
                    return Err(I2cError {
                        kind: ErrorKind::Bus,
                    });
                }
            }

            *byte = self.i2c.rxdr().read().rxdata();
        }

        Ok(())
    }
}
