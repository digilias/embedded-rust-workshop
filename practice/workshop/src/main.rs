#![no_std]
#![no_main]

use embassy_stm32::i2c::{Config, Error, I2c};
use embassy_stm32::pac;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::{
    mode::Async,
    bind_interrupts, exti,
    gpio::{Input, Pull, Output, Level, Speed},
    i2c, interrupt, peripherals,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_sync::channel::{Channel, Sender, Receiver};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_executor::Spawner;
use lis3dh_async::{Lis3dh, SlaveAddr, Configuration, Lis3dhCore, Register, Lis3dhI2C, DataRate};

bind_interrupts!(pub struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    EXTI13 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI13>;
    EXTI5 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI5>;
});

#[embassy_executor::main]
async fn main(s: Spawner) {
    // Initialize HAL
    let p = embassy_stm32::init(Default::default());

    let mut enable = Output::new(p.PG9, Level::Low, Speed::Low);
    let mut irq = ExtiInput::new(p.PA5, p.EXTI5, Pull::Down, Irqs);

    enable.set_high();
    Timer::after_secs(1).await;

    // Create an i2c instance
    let mut config = Config::default();
    config.timeout = Duration::from_secs(2);
    let i2c = I2c::new(p.I2C1, p.PB8, p.PB9, Irqs, p.GPDMA1_CH4, p.GPDMA1_CH5, config);


    let mut device = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, Configuration::default()).await.unwrap();

    let val = device.read_register(Register::WHOAMI).await.unwrap();
    defmt::info!("whoami: {}", val);

    static CHANNEL: Channel<ThreadModeRawMutex, Sample, 10> = Channel::new();
    s.spawn(producer(device, CHANNEL.sender(), irq).unwrap());
    s.spawn(consumer(CHANNEL.receiver()).unwrap());
}

use lowpass_filter_sys::{Sample, LowpassFilter, lowpass_filter_init, lowpass_filter_apply};

#[embassy_executor::task]
async fn producer(mut xl: Lis3dh<Lis3dhI2C<I2c<'static, Async, i2c::Master>>>, sender: Sender<'static, ThreadModeRawMutex, Sample, 10>, mut irq: ExtiInput<'static>) {
    use defmt::unwrap;
    use lis3dh_async::{Interrupt1, InterruptMode, InterruptConfig, IrqPin1Config};

    let dr = DataRate::Hz_1;
    unwrap!(xl.set_datarate(dr).await);

    unwrap!(xl.configure_irq_src(
        Interrupt1,
        InterruptMode::Position,
        InterruptConfig::high_and_low(),
    ).await);

    // Raise pin state if interrupt 1 is raised and there is movement
    unwrap!(xl.configure_interrupt_pin(IrqPin1Config {
        zyxda_en: true,
        ..IrqPin1Config::default()
    }).await);


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
}

#[embassy_executor::task]
async fn consumer(receiver: Receiver<'static, ThreadModeRawMutex, Sample, 10>) {
    loop {
        let sample = receiver.receive().await;
        defmt::info!("x: {}, y: {}, z: {}", sample.x, sample.y, sample.z);
    }

}
