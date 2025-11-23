#ifndef MATHLIB_H
#define MATHLIB_H

#include <stdint.h>

// Initialize the math library
void mathlib_init(void);

// Cleanup the math library
void mathlib_cleanup(void);

// Calculate CRC8
// Returns: 0 on success, negative error code on failure
int32_t mathlib_crc8(const uint8_t* data, uint32_t length, uint8_t* result);

// Fixed-point sine calculation
// Input: angle in degrees (0-359)
// Returns: sine value scaled by 1000, or negative error code
int32_t mathlib_sin_fixed(int32_t degrees);

#endif // MATHLIB_H
