#include "lowpass_filter.h"

void lowpass_filter_init(LowpassFilter *filter, float alpha)
{
    if (!filter) {
        return;
    }

    filter->has_state = 0;
    filter->alpha = alpha;
}

Sample lowpass_filter_apply(LowpassFilter *filter, Sample raw_sample)
{
    if (!filter) {
        return raw_sample;
    }

    if (filter->has_state == 0) {
        /* First sample initializes the filter */
        filter->state = raw_sample;
        filter->has_state = 1;
        return raw_sample;
    }

    /* Exponential moving average */
    Sample filtered;
    float a = filter->alpha;
    float one_minus_a = 1.0f - a;

    filtered.x = a * raw_sample.x + one_minus_a * filter->state.x;
    filtered.y = a * raw_sample.y + one_minus_a * filter->state.y;
    filtered.z = a * raw_sample.z + one_minus_a * filter->state.z;

    filter->state = filtered;
    return filtered;
}
