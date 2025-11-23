// Session 2 Snippet: LIS3DH Register Definitions

// I2C Address
pub const LIS3DH_ADDR: u8 = 0x18;  // 7-bit address (SDO/SA0 to GND)
// If SDO/SA0 is connected to VCC, use 0x19

// Register Addresses
pub const WHO_AM_I: u8 = 0x0F;
pub const CTRL_REG1: u8 = 0x20;
pub const CTRL_REG2: u8 = 0x21;
pub const CTRL_REG3: u8 = 0x22;
pub const CTRL_REG4: u8 = 0x23;
pub const CTRL_REG5: u8 = 0x24;
pub const CTRL_REG6: u8 = 0x25;

pub const STATUS_REG: u8 = 0x27;

pub const OUT_X_L: u8 = 0x28;
pub const OUT_X_H: u8 = 0x29;
pub const OUT_Y_L: u8 = 0x2A;
pub const OUT_Y_H: u8 = 0x2B;
pub const OUT_Z_L: u8 = 0x2C;
pub const OUT_Z_H: u8 = 0x2D;

pub const INT1_CFG: u8 = 0x30;
pub const INT1_SRC: u8 = 0x31;
pub const INT1_THS: u8 = 0x32;
pub const INT1_DURATION: u8 = 0x33;

// Expected Values
pub const EXPECTED_WHO_AM_I: u8 = 0x33;

// CTRL_REG1 bits
pub const ODR_POWERDOWN: u8 = 0b0000_0000;
pub const ODR_1HZ: u8 = 0b0001_0000;
pub const ODR_10HZ: u8 = 0b0010_0000;
pub const ODR_25HZ: u8 = 0b0011_0000;
pub const ODR_50HZ: u8 = 0b0100_0000;
pub const ODR_100HZ: u8 = 0b0101_0000;

pub const XYZEN: u8 = 0b0000_0111;  // Enable all axes
