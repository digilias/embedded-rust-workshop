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


    let mut config = Configuration::default();
    config.datarate = DataRate::Hz_400;
    let mut device = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, config).await.unwrap();

    let val = device.read_register(Register::WHOAMI).await.unwrap();
    defmt::info!("whoami: {}", val);

    static CHANNEL: Channel<ThreadModeRawMutex, Sample, 10> = Channel::new();
    s.spawn(producer(device, CHANNEL.sender(), irq).unwrap());
    s.spawn(consumer(CHANNEL.receiver()).unwrap());
}

struct Sample {
    x: f32,
    y: f32,
    z: f32,
}


#[embassy_executor::task]
async fn producer(mut xl: Lis3dh<Lis3dhI2C<I2c<'static, Async, i2c::Master>>>, sender: Sender<'static, ThreadModeRawMutex, Sample, 10>, mut irq: ExtiInput<'static>) {
    use defmt::unwrap;
    use lis3dh_async::{Interrupt1, InterruptMode, InterruptConfig, IrqPin1Config, Threshold, Range, Detect4D, LatchInterruptRequest};

    // configurations for control registers
//   unwrap!(xl.write_register(Register::CTRL1, 0xA7).await);
//   unwrap!(xl.write_register(Register::CTRL3, 0x40).await);
//   unwrap!(xl.write_register(Register::CTRL4, 0x00).await);
//   unwrap!(xl.write_register(Register::CTRL5, 0x08).await);
//
//   unwrap!(xl.write_register(Register::INT1_THS, 0x10).await);
//   unwrap!(xl.write_register(Register::INT1_DURATION, 0x00).await);
//   unwrap!(xl.write_register(Register::INT1_CFG, 0x2A).await);

//
//    writeRegister(0x20, 0xA7); //Write A7h into CTRL_REG1;      // Turn on the sensor, enable X, Y, Z axes with ODR = 100Hz normal mode.
//    writeRegister(0x21, 0x09); //Write 09h into CTRL_REG2;      // High-pass filter (HPF) enabled
//    writeRegister(0x22, 0x40); //Write 40h into CTRL_REG3;      // ACC AOI1 interrupt signal is routed to INT1 pin.
//    writeRegister(0x23, 0x00); //Write 00h into CTRL_REG4;      // Full Scale = +/-2 g
//    writeRegister(0x24, 0x08); //Write 08h into CTRL_REG5;      // Default value is 00 for no latching. Interrupt signals on INT1 pin is not latched.
//                                                                //Users don’t need to read the INT1_SRC register to clear the interrupt signal.
//    // configurations for wakeup and motionless detection
//    writeRegister(0x32, 0x10); //Write 10h into INT1_THS;          // Threshold (THS) = 16LSBs * 15.625mg/LSB = 250mg.
//    writeRegister(0x33, 0x00); //Write 00h into INT1_DURATION;     // Duration = 1LSBs * (1/10Hz) = 0.1s.
//    //readRegister();  //Dummy read to force the HP filter to set reference acceleration/tilt value
//    writeRegister(0x30, 0x2A); //Write 2Ah into INT1_CFG;          // Enable XLIE, YLIE, ZLIE interrupt generation, OR logic.

    let dr = DataRate::Hz_400;
    let duration = lis3dh_async::Duration::seconds(dr, 0.0001);

    let threshold = Threshold::g(Range::G16, 0.250);
    unwrap!(xl.configure_irq_threshold(Interrupt1, threshold).await);

    unwrap!(xl.configure_irq_duration(Interrupt1, duration).await);

    unwrap!(xl.configure_irq_src_and_control(
        Interrupt1,
        InterruptMode::Movement,
        InterruptConfig::high_and_low(),
        LatchInterruptRequest::Disable,
        Detect4D::Enable,
    ).await);

    unwrap!(xl.configure_interrupt_pin(IrqPin1Config {
        ia1_en: true,
         zyxda_en: true,
        ..IrqPin1Config::default()
    }).await);

     unwrap!(xl.write_register(Register::CTRL2, 0x09).await);

    loop {
        let _ = irq.wait_for_high().await;
        let s = xl.accel_norm().await.unwrap();
        sender.send(Sample {
            x: s.x,
            y: s.y,
            z: s.z,
        }).await;
        let s = unwrap!(xl.get_irq_src(Interrupt1).await);
    }
}

#[embassy_executor::task]
async fn consumer(receiver: Receiver<'static, ThreadModeRawMutex, Sample, 10>) {
    loop {
        let sample = receiver.receive().await;
        defmt::info!("x: {}, y: {}, z: {}", sample.x, sample.y, sample.z);
    }

}
