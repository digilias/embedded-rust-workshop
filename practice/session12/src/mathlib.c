#include <stdint.h>

// Simple math library for demonstration purposes

// Initialize the math library (currently does nothing, but demonstrates the pattern)
void mathlib_init(void) {
    // Initialization code would go here
}

// Cleanup the math library
void mathlib_cleanup(void) {
    // Cleanup code would go here
}

// Simple CRC8 calculation
// Returns: CRC8 value
// Error codes: negative values indicate errors
int32_t mathlib_crc8(const uint8_t* data, uint32_t length, uint8_t* result) {
    if (data == NULL || result == NULL) {
        return -1;  // NULL pointer error
    }

    if (length == 0) {
        return -2;  // Invalid length
    }

    uint8_t crc = 0xFF;

    for (uint32_t i = 0; i < length; i++) {
        crc ^= data[i];

        for (uint8_t bit = 0; bit < 8; bit++) {
            if (crc & 0x80) {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc = crc << 1;
            }
        }
    }

    *result = crc;
    return 0;  // Success
}

// Fixed-point sine approximation (returns sin(x) * 1000 for integer math)
// Input: angle in degrees (0-359)
// Output: sine value scaled by 1000 (-1000 to 1000)
// Error codes: negative values indicate errors
int32_t mathlib_sin_fixed(int32_t degrees) {
    if (degrees < 0 || degrees >= 360) {
        return -32768;  // Error: out of range
    }

    // Simple lookup table for demonstration
    // In a real application, you might use a more sophisticated approximation
    static const int16_t sin_table[] = {
        0, 17, 35, 52, 70, 87, 105, 122, 139, 156,  // 0-9 degrees
        174, 191, 208, 225, 242, 259, 276, 292, 309, 326,  // 10-19
        342, 358, 375, 391, 407, 423, 438, 454, 469, 485,  // 20-29
        500, 515, 530, 545, 559, 574, 588, 602, 616, 629,  // 30-39
        643, 656, 669, 682, 695, 707, 719, 731, 743, 755,  // 40-49
        766, 777, 788, 799, 809, 819, 829, 839, 848, 857,  // 50-59
        866, 875, 883, 891, 899, 906, 914, 921, 927, 934,  // 60-69
        940, 946, 951, 956, 961, 966, 970, 974, 978, 982,  // 70-79
        985, 988, 990, 993, 995, 996, 998, 999, 999, 1000  // 80-89
    };

    int32_t angle = degrees;
    int32_t sign = 1;

    // Normalize to 0-89 degrees
    if (angle >= 270) {
        angle = 360 - angle;
        sign = -1;
    } else if (angle >= 180) {
        angle = angle - 180;
        sign = -1;
    } else if (angle >= 90) {
        angle = 180 - angle;
    }

    return sign * sin_table[angle];
}
