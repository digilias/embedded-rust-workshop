#![no_std]
#![no_main]

use embassy_stm32::i2c::{Config, Error, I2c};
use embassy_stm32::pac;
use embassy_stm32::rng;
use embassy_stm32::eth;
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
use lowpass_filter_sys::{Sample, LowpassFilter, lowpass_filter_init, lowpass_filter_apply};
use embassy_stm32::peripherals::{ETH_SMA, ETH};
use embassy_stm32::eth::{Ethernet, GenericPhy, PacketQueue, Sma};
use embassy_net::{Stack, StackResources};
use static_cell::StaticCell;
use lis3dh_async::{Interrupt1, InterruptMode, InterruptConfig, IrqPin1Config};
use embassy_net::tcp::TcpSocket;
use defmt::unwrap;
use core::net::SocketAddr;
use core::net::IpAddr;
use core::net::Ipv4Addr;
use embedded_io_async::Write;

bind_interrupts!(pub struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    EXTI13 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI13>;
    EXTI5 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI5>;
    RNG => rng::InterruptHandler<peripherals::RNG>;
    ETH => eth::InterruptHandler;
});

#[embassy_executor::main]
async fn main(s: Spawner) {
    // Initialize HAL
    let p = embassy_stm32::init(Default::default());

    let irq = ExtiInput::new(p.PA5, p.EXTI5, Pull::Down, Irqs);

    // Create an i2c instance
    let mut config = Config::default();
    config.timeout = Duration::from_secs(2);
    let i2c = I2c::new(p.I2C1, p.PB8, p.PB9, Irqs, p.GPDMA1_CH4, p.GPDMA1_CH5, config);

    let mut device = Lis3dh::new_i2c_with_config(i2c, SlaveAddr::Default, Configuration::default()).await.unwrap();

    let val = device.read_register(Register::WHOAMI).await.unwrap();
    defmt::info!("whoami: {}", val);

    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    let eth = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.ETH,
        Irqs,
        p.PA1,
        p.PA7,
        p.PC4,
        p.PC5,
        p.PG13,
        p.PB15,
        p.PG11,
        mac_addr,
        p.ETH_SMA,
        p.PA2,
        p.PC1,
    );

    // Generate random seed.
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let config = embassy_net::Config::dhcpv4(Default::default());

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(eth, config, RESOURCES.init(StackResources::new()), seed);

    s.spawn(net_task(runner).unwrap());

    // Ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;


    static CHANNEL: Channel<ThreadModeRawMutex, Sample, 10> = Channel::new();
    s.spawn(producer(device, CHANNEL.sender(), irq).unwrap());
    s.spawn(consumer(CHANNEL.receiver(), stack).unwrap());
}


type Device = Ethernet<'static, ETH, GenericPhy<Sma<'static, ETH_SMA>>>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn producer(mut xl: Lis3dh<Lis3dhI2C<I2c<'static, Async, i2c::Master>>>, sender: Sender<'static, ThreadModeRawMutex, Sample, 10>, mut irq: ExtiInput<'static>) {

    let dr = DataRate::Hz_100;
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
async fn consumer(receiver: Receiver<'static, ThreadModeRawMutex, Sample, 10>, stack: embassy_net::Stack<'static>) {

    let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 162, 130)), 8080);

    let mut tx_buffer = [0u8; 2048];
    let mut rx_buffer = [0u8; 2048];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    unwrap!(socket.connect(remote).await);

    loop {
        let sample = receiver.receive().await;
        defmt::info!("x: {}, y: {}, z: {}", sample.x, sample.y, sample.z);

        let (x, y, z) = (sample.x.to_le_bytes(), sample.y.to_le_bytes(), sample.z.to_le_bytes());
        unwrap!(socket.write_all(&x).await);
        unwrap!(socket.write_all(&y).await);
        unwrap!(socket.write_all(&z).await);

    }

}
