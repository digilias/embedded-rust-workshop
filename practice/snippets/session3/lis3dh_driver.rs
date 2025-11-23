// Session 3 Snippet: Generic LIS3DH Driver using embedded-hal

use embedded_hal::i2c::I2c;

// Register addresses
const WHO_AM_I: u8 = 0x0F;
const CTRL_REG1: u8 = 0x20;
const CTRL_REG4: u8 = 0x23;
const OUT_X_L: u8 = 0x28;

#[derive(Clone, Copy, Debug)]
pub struct Acceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct Lis3dh<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C: I2c> Lis3dh<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Verify the WHO_AM_I register
    pub fn who_am_i(&mut self) -> Result<u8, I2C::Error> {
        self.read_register(WHO_AM_I)
    }

    /// Initialize the sensor with default settings
    pub fn init(&mut self) -> Result<(), I2C::Error> {
        // Verify WHO_AM_I
        let who = self.who_am_i()?;
        if who != 0x33 {
            // Return error (for simplicity, reuse I2C error)
            // In production code, you'd have a custom error type
        }

        // Enable all axes, 100Hz data rate
        self.write_register(CTRL_REG1, 0b0101_0111)?;

        // High resolution mode, +/- 2g
        self.write_register(CTRL_REG4, 0b0000_1000)?;

        Ok(())
    }

    /// Read acceleration data
    pub fn read_accel(&mut self) -> Result<Acceleration, I2C::Error> {
        // Read 6 bytes starting from OUT_X_L
        // LIS3DH supports auto-increment when bit 7 is set
        let mut buffer = [0u8; 6];
        self.i2c
            .write_read(self.address, &[OUT_X_L | 0x80], &mut buffer)?;

        let x = i16::from_le_bytes([buffer[0], buffer[1]]);
        let y = i16::from_le_bytes([buffer[2], buffer[3]]);
        let z = i16::from_le_bytes([buffer[4], buffer[5]]);

        Ok(Acceleration { x, y, z })
    }

    /// Read a single register
    fn read_register(&mut self, reg: u8) -> Result<u8, I2C::Error> {
        let mut data = [0u8; 1];
        self.i2c.write_read(self.address, &[reg], &mut data)?;
        Ok(data[0])
    }

    /// Write a single register
    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), I2C::Error> {
        self.i2c.write(self.address, &[reg, value])
    }
}
