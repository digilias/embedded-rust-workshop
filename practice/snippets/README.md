# Workshop Code Snippets

This directory contains reference code snippets for each workshop session. These are meant to help you when you get stuck, but try to implement things yourself first!

## How to Use Snippets

1. **Try implementing yourself first** - The snippets are here to help, not to copy blindly
2. **Understand before copying** - Read through the snippet and understand what it does
3. **Adapt to your code** - The snippets might need slight modifications for your specific implementation

## Session Guide

### Session 2: PAC - Direct Register Access
- `i2c_pin_config.rs` - GPIO configuration for I2C pins
- `i2c_init.rs` - I2C peripheral initialization
- `lis3dh_regs.rs` - LIS3DH register definitions
- `lis3dh_read.rs` - Complete I2C read transaction

### Session 3: embedded-hal - Platform-Agnostic Drivers
- `i2c_hal_impl.rs` - Implementing embedded-hal I2c trait
- `lis3dh_driver.rs` - Generic LIS3DH driver using embedded-hal

### Session 4: Embassy HAL
- `embassy_i2c.rs` - Using embassy-stm32 I2C (blocking)
- `embassy_async_i2c.rs` - Using embassy-stm32 I2C with async

### Session 5: Interrupts
- `lis3dh_interrupt_config.rs` - Configuring LIS3DH interrupt registers
- `exti_interrupt.rs` - EXTI interrupt handler setup

### Session 6: Async Fundamentals
- `simple_executor.rs` - Educational executor implementation

### Session 7: Embassy Executor
- `embassy_tasks.rs` - Task spawning and management examples

### Session 8: Async + Interrupts
- `exti_input_async.rs` - Using ExtiInput for async interrupt handling

### Session 9: Synchronization
- `channel_producer_consumer.rs` - Producer-consumer pattern with Channel
- `signal_example.rs` - Using Signal for event notification

### Session 10: Multiple Executors
- `interrupt_executor.rs` - High-priority InterruptExecutor setup

### Session 11: Panic & Logging
- `panic_handler.rs` - Custom panic handler with persistent state

### Session 13: Networking
- `ethernet_init.rs` - Ethernet and network stack initialization
- `tcp_client.rs` - TCP client for sending sensor data

## Tips

- **Read the comments** - Each snippet has explanatory comments
- **Check the workshop README** - The main workshop README tells you which snippets to reference for each session
- **Mix and match** - Combine ideas from multiple snippets
- **Ask questions** - If a snippet doesn't make sense, ask the instructor!

## Not Just Copy-Paste

These snippets are learning tools. The goal is to understand:
- **Why** the code is written this way
- **How** it fits into the bigger picture
- **What** trade-offs are being made

Try to implement features yourself before looking at snippets. When you do look, understand the approach rather than just copying code.
