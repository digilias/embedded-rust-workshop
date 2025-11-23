# Workshop: Embedded Rust Accelerometer Application

This is your work-in-progress application that you'll build throughout the workshop.

## Current Session: 2 - PAC

### Goal
Read the WHO_AM_I register (0x0F) from the LIS3DH accelerometer using stm32-metapac I2C.

### Hardware Setup
- **Board**: STM32H563ZI Nucleo
- **Sensor**: LIS3DH accelerometer
- **I2C**: I2C1 on PB8 (SCL) and PB9 (SDA)
- **Address**: 0x18 (7-bit)

### Tasks

1. **Enable clocks**
   - Enable GPIOB clock
   - Enable I2C1 clock

2. **Configure I2C pins**
   - Set PB8 and PB9 to alternate function (AF4)
   - Configure as open-drain with pull-ups
   - See `snippets/session2/i2c_pin_config.rs`

3. **Initialize I2C peripheral**
   - Configure timing for 100kHz
   - Enable peripheral
   - See `snippets/session2/i2c_init.rs`

4. **Read WHO_AM_I**
   - Write register address (0x0F)
   - Read one byte
   - Should return 0x33
   - See `snippets/session2/lis3dh_read.rs`

### Expected Output

```
INFO  Workshop: Embedded Rust with LIS3DH Accelerometer
INFO  Session 2: Reading WHO_AM_I using PAC
INFO  WHO_AM_I: 0x33 ✓
```

### Snippets Available

- `snippets/session2/i2c_pin_config.rs` - GPIO configuration for I2C
- `snippets/session2/i2c_init.rs` - I2C peripheral initialization
- `snippets/session2/lis3dh_regs.rs` - LIS3DH register definitions
- `snippets/session2/lis3dh_read.rs` - Complete read transaction

### Building and Running

```bash
cd practice/workshop
cargo build
cargo run
```

### Troubleshooting

**I2C timeout?**
- Check wiring (SCL=PB8, SDA=PB9)
- Verify pull-up resistors present
- Check sensor power (3.3V)

**Wrong WHO_AM_I value?**
- Verify I2C address (0x18 vs 0x19)
- Check sensor datasheet for correct value

**Compile errors?**
- Make sure you're using `pac::GPIOB` not just `GPIOB`
- Import necessary vals: `use pac::gpio::vals;`

### Next Session

Session 3: We'll create an embedded-hal I2C wrapper and a generic LIS3DH driver.
