use embedded_hal::i2c::I2c;

// Sensor register addresses (example for a temperature sensor)
const WHO_AM_I: u8 = 0x0F;
const CTRL_REG: u8 = 0x20;
const TEMP_OUT_L: u8 = 0x2C;
const TEMP_OUT_H: u8 = 0x2D;

pub struct TemperatureSensor<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C: I2c> TemperatureSensor<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    // TODO: Implement init method
    // pub fn init(&mut self) -> Result<(), I2C::Error> {
    //     // 1. Read WHO_AM_I register to verify device
    //     // 2. Configure control register to enable sensor
    //     todo!()
    // }

    // TODO: Implement read_temperature method
    // pub fn read_temperature(&mut self) -> Result<i32, I2C::Error> {
    //     // 1. Read temperature low byte
    //     // 2. Read temperature high byte
    //     // 3. Combine bytes and convert to temperature in 0.01°C units
    //     todo!()
    // }

    // TODO: Helper method to read a single register
    // fn read_register(&mut self, reg: u8) -> Result<u8, I2C::Error> {
    //     let mut data = [0u8; 1];
    //     self.i2c.write_read(self.address, &[reg], &mut data)?;
    //     Ok(data[0])
    // }

    // TODO: Helper method to write a single register
    // fn write_register(&mut self, reg: u8, value: u8) -> Result<(), I2C::Error> {
    //     self.i2c.write(self.address, &[reg, value])
    // }
}
