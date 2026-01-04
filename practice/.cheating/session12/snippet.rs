// NOTE: Sample replaces the existing.
use lowpass_filter_sys::{Sample, LowpassFilter, lowpass_filter_init, lowpass_filter_apply};
use core::mem::MaybeUninit;
let mut filter: MaybeUninit<LowpassFilter> = MaybeUninit::uninit();

unsafe { lowpass_filter_init(filter.as_mut_ptr(), 0.2) };
loop {
    let _ = irq.wait_for_high().await;
    let s = xl.accel_norm().await.unwrap();
    let s = Sample {
        x: s.x,
        y: s.y,
        z: s.z,
    };
    let filtered = unsafe { lowpass_filter_apply(filter.as_mut_ptr(), s) };

    sender.send(filtered).await;
}
