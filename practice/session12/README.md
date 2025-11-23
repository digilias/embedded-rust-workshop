# Session 12: C Library Integration

This session demonstrates how to safely wrap C library code in Rust for use in embedded systems.

## Goal

Learn to integrate C libraries with Rust by:
- Using the `cc` crate to build C code
- Creating safe Rust wrappers around unsafe C functions
- Handling C-style error codes properly
- Managing memory and pointers safely

## What's Provided

- `src/mathlib.c` and `src/mathlib.h`: A simple C math library with:
  - `mathlib_init()`: Initialize the library
  - `mathlib_cleanup()`: Cleanup resources
  - `mathlib_crc8()`: Calculate CRC8 checksum (with C-style error handling)
  - `mathlib_sin_fixed()`: Fixed-point sine calculation

- `build.rs`: Build script that compiles the C library using the `cc` crate

- `src/main.rs`: Skeleton code with TODOs for you to complete

## Your Tasks

### 1. Declare C Functions
Add an `extern "C"` block to declare the C functions from `mathlib.h`

### 2. Create Safe Wrappers
Implement safe Rust wrapper functions:
- `calculate_crc8(data: &[u8]) -> Result<u8, MathLibError>`
  - Convert Rust slice to C pointer and length
  - Call unsafe C function
  - Convert C error codes to Rust Result

- `calculate_sine(degrees: i32) -> Result<i32, MathLibError>`
  - Validate input range
  - Call unsafe C function
  - Handle error return values

### 3. Error Handling
Complete the `MathLibError` enum to represent all possible C library errors:
- Null pointer errors
- Invalid length
- Out of range errors
- Unknown errors

Implement a helper to convert C error codes to `MathLibError`

### 4. Test on Hardware
- Uncomment the test code in `main()`
- Build and run on your STM32 board
- Verify CRC8 and sine calculations work correctly

## Key Concepts

### The `unsafe` Keyword
All FFI calls to C are `unsafe` because Rust cannot verify their safety:
```rust
unsafe {
    mathlib_crc8(ptr, len, result_ptr)
}
```

### Creating Safe Abstractions
Your safe wrapper should:
```rust
fn calculate_crc8(data: &[u8]) -> Result<u8, MathLibError> {
    let mut result = 0u8;
    let ret = unsafe {
        mathlib_crc8(
            data.as_ptr(),
            data.len() as u32,
            &mut result as *mut u8
        )
    };

    if ret < 0 {
        Err(MathLibError::from_code(ret))
    } else {
        Ok(result)
    }
}
```

### Memory Safety Considerations
- Ensure pointers are valid (non-null, properly aligned)
- Respect lifetimes (C function shouldn't store the pointer)
- Validate lengths before passing to C
- Handle C's error codes properly

## Building and Running

```bash
cargo build
cargo run
```

The output should show:
- CRC8 calculation results
- Sine values for various angles

## Extension Challenges

1. Add more C functions (e.g., `mathlib_cos_fixed`)
2. Create a zero-cost abstraction using a newtype
3. Add benchmarking to compare C vs pure Rust implementations
4. Implement `From` trait for error conversion
5. Add more comprehensive error handling
