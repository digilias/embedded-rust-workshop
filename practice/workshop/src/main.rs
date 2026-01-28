#![no_std]
#![no_main]

use embassy_stm32::gpio::{Level, Output, Speed};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    defmt::info!("Hello World!");

    let mut led = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        defmt::info!("high");
        led.set_high();

        for _i in 0..10000000 {
            cortex_m::asm::nop();
        }

        defmt::info!("low");
        led.set_low();

        for _i in 0..10000000 {
            cortex_m::asm::nop();
        }
    }
}
