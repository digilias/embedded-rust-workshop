#ifndef LOWPASS_FILTER_H
#define LOWPASS_FILTER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Equivalent to Rust's Sample struct */
typedef struct {
    float x;
    float y;
    float z;
} Sample;

/* Low-pass filter state */
typedef struct {
    uint8_t has_state;
    Sample state;
    float alpha;
} LowpassFilter;

/**
 * Initialize a low-pass filter.
 *
 * @param filter Pointer to filter instance
 * @param alpha  Smoothing factor (0.0 .. 1.0)
 */
void lowpass_filter_init(LowpassFilter *filter, float alpha);

/**
 * Apply the low-pass filter to a sample.
 *
 * @param filter     Pointer to filter instance
 * @param raw_sample New input sample
 * @return           Filtered sample
 */
Sample lowpass_filter_apply(LowpassFilter *filter, Sample raw_sample);

#ifdef __cplusplus
}
#endif

#endif /* LOWPASS_FILTER_H */
