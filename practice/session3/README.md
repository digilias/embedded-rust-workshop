# Session 3: Platform-Agnostic Drivers with embedded-hal

This session demonstrates how to create platform-agnostic device drivers using embedded-hal traits.

## Goal

Learn to write portable drivers by:
- Implementing embedded-hal traits for I2C
- Creating a generic sensor driver that works with any I2C implementation
- Understanding the benefits of trait-based abstraction
- Making code reusable across different platforms

## What's Provided

- `src/main.rs`: Main program with I2C pin configuration
- `src/i2c_impl.rs`: Skeleton for embedded-hal I2C implementation using stm32-metapac
- `src/sensor.rs`: Skeleton for a generic temperature sensor driver

## Your Tasks

### 1. Complete I2C Implementation
In `src/i2c_impl.rs`:
- Implement helper methods for I2C status checking (wait_busy, wait_txis, etc.)
- Implement the `embedded_hal::i2c::I2c` trait:
  - `read()`: Read bytes from device
  - `write()`: Write bytes to device
  - `write_read()`: Write then read without stop condition
  - `transaction()` (optional): Execute multiple operations

Reference the session2 code for the low-level I2C operations.

### 2. Complete Sensor Driver
In `src/sensor.rs`:
- Implement `init()`: Initialize and verify the sensor
- Implement `read_temperature()`: Read temperature from sensor
- Add helper methods for register read/write

The driver should be generic over any `I2c` implementation!

### 3. Test the Driver
In `src/main.rs`:
- Create the I2C bus instance
- Create the sensor driver using the I2C bus
- Initialize the sensor
- Periodically read and display temperature

## Key Concepts

### embedded-hal Traits
```rust
pub trait I2c: ErrorType {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error>;
    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8])
        -> Result<(), Self::Error>;
}
```

### Generic Driver Pattern
```rust
pub struct TemperatureSensor<I2C> {
    i2c: I2C,  // Generic over any I2C implementation
}

impl<I2C: I2c> TemperatureSensor<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}
```

This driver works with:
- Your custom stm32-metapac implementation
- embassy-stm32's I2C
- Linux i2c-dev
- Mock implementations for testing!

### Benefits
- **Portability**: Same driver works on different platforms
- **Testability**: Can use mock I2C for unit tests
- **Reusability**: Share drivers across projects
- **Type Safety**: Compiler ensures correct trait usage

## Building and Running

```bash
cargo build
cargo run
```

Expected output:
```
INFO  Session 3: Platform-Agnostic Drivers
INFO  Sensor initialized
INFO  Temperature: 23.45 °C
INFO  Temperature: 23.46 °C
```

## Extension Challenges

1. **Add more sensor methods**:
   - Read humidity (if your sensor supports it)
   - Configure sampling rate
   - Enable/disable sensor

2. **Use device-driver crate**:
   - Define registers using `device-driver` macros
   - Reduces boilerplate for register access

3. **Add async support**:
   - Implement `embedded_hal_async::i2c::I2c` trait
   - Make sensor driver work with both sync and async

4. **Bus sharing**:
   - Use `embedded-hal-bus` or `embassy-embedded-hal` for bus sharing
   - Connect multiple devices on the same I2C bus

5. **Error handling**:
   - Add custom error type with more context
   - Implement proper error recovery

## Comparison with Session 2

**Session 2 (PAC)**:
- Direct register manipulation
- Platform-specific code
- No abstraction

**Session 3 (embedded-hal)**:
- Trait-based abstraction
- Platform-agnostic drivers
- Reusable and testable

The embedded-hal approach requires more upfront work but pays off in:
- Code reuse
- Easier testing
- Better separation of concerns
