// Session 5 Snippet: Configuring LIS3DH Interrupts

// LIS3DH Interrupt Registers
const INT1_CFG: u8 = 0x30;
const INT1_THS: u8 = 0x32;
const INT1_DURATION: u8 = 0x33;
const CTRL_REG3: u8 = 0x22;
const CTRL_REG5: u8 = 0x24;

/// Configure LIS3DH to generate interrupt on data ready
pub fn configure_data_ready_interrupt<I2C: embedded_hal::i2c::I2c>(
    sensor: &mut impl SensorI2c<I2C>,
) -> Result<(), I2C::Error> {
    // Enable interrupt 1 on INT1 pin for data ready
    // CTRL_REG3: I1_ZYXDA = 1 (bit 4)
    sensor.write_register(CTRL_REG3, 0b0001_0000)?;

    // CTRL_REG5: Latch interrupt request
    sensor.write_register(CTRL_REG5, 0b0000_1000)?;

    Ok(())
}

/// Configure LIS3DH to generate interrupt on movement detection
pub fn configure_movement_interrupt<I2C: embedded_hal::i2c::I2c>(
    sensor: &mut impl SensorI2c<I2C>,
    threshold_mg: u16,
) -> Result<(), I2C::Error> {
    // INT1_CFG: Enable X, Y, Z high events
    sensor.write_register(INT1_CFG, 0b0010_1010)?;

    // INT1_THS: Threshold in mg/16 (for 2g scale)
    let ths = (threshold_mg / 16) as u8;
    sensor.write_register(INT1_THS, ths)?;

    // INT1_DURATION: Minimum duration
    sensor.write_register(INT1_DURATION, 0)?;

    // CTRL_REG3: Route interrupt 1 to INT1 pin
    sensor.write_register(CTRL_REG3, 0b0100_0000)?;

    Ok(())
}

trait SensorI2c<I2C: embedded_hal::i2c::I2c> {
    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), I2C::Error>;
}
