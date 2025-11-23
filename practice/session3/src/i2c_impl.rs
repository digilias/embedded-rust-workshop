use embedded_hal::i2c::{I2c, Error, ErrorKind, ErrorType};
use stm32_metapac as pac;

pub struct I2cBus {
    i2c: pac::i2c::I2c,
    address: u8,
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
    pub fn new(i2c: pac::i2c::I2c, address: u8) -> Self {
        // Configure I2C timing for 100kHz with 64MHz clock
        i2c.cr1().modify(|w| w.set_pe(false));

        i2c.timingr().write(|w| {
            w.set_presc(15);
            w.set_scll(0x13);
            w.set_sclh(0x0F);
            w.set_sdadel(0x02);
            w.set_scldel(0x04);
        });

        i2c.cr1().modify(|w| w.set_pe(true));

        Self { i2c, address }
    }

    // TODO: Implement helper methods for I2C operations
    // fn wait_busy(&self) -> Result<(), I2cError> { ... }
    // fn wait_txis(&self) -> Result<(), I2cError> { ... }
    // fn wait_tc(&self) -> Result<(), I2cError> { ... }
    // fn wait_rxne(&self) -> Result<(), I2cError> { ... }
}

impl ErrorType for I2cBus {
    type Error = I2cError;
}

// TODO: Implement the embedded_hal::i2c::I2c trait
// impl I2c for I2cBus {
//     fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
//         // Implement I2C read operation
//         todo!()
//     }
//
//     fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
//         // Implement I2C write operation
//         todo!()
//     }
//
//     fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
//         // Implement I2C write-read operation (write then read without stop)
//         todo!()
//     }
//
//     fn transaction(
//         &mut self,
//         address: u8,
//         operations: &mut [embedded_hal::i2c::Operation<'_>],
//     ) -> Result<(), Self::Error> {
//         // Implement transaction (optional but useful)
//         todo!()
//     }
// }
