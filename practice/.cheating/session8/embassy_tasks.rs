bind_interrupts!(pub struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    EXTI13 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI13>;
});

#[embassy_executor::main]
async fn main(s: Spawner) {
    // Initialize HAL
    let p = embassy_stm32::init(Default::default());

    // Create an i2c instance
    let mut config = Config::default();
    config.timeout = Duration::from_secs(2);
    let i2c = I2c::new(p.I2C1, p.PB8, p.PB9, Irqs, p.GPDMA1_CH4, p.GPDMA1_CH5, config);

    let mut device = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, Configuration::default()).await.unwrap();

    let val = device.read_register(Register::WHOAMI).await.unwrap();
    defmt::info!("whoami: {}", val);

    let mut button = exti::ExtiInput::new(p.PC13, p.EXTI13, Pull::Down, Irqs);
    loop {
        button.wait_for_rising_edge().await;
        defmt::info!("Hello!");
    }
}
