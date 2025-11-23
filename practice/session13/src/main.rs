#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources, Config};
use embassy_stm32::eth::{Ethernet, PacketQueue};
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::peripherals::ETH;
use embassy_stm32::rcc::{AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllSource, Sysclk, VoltageScale};
use embassy_stm32::{bind_interrupts, eth, Config as StmConfig};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Network buffer sizes
const RX_QUEUE_SIZE: usize = 4;
const TX_QUEUE_SIZE: usize = 4;

// TODO: Set your backend server address
// const SERVER_ADDRESS: (u8, u8, u8, u8) = (192, 168, 1, 100);
// const SERVER_PORT: u16 = 8080;

// Bind Ethernet interrupt
bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

// Static allocations for network stack
static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
static RX_QUEUE: StaticCell<PacketQueue<RX_QUEUE_SIZE>> = StaticCell::new();
static TX_QUEUE: StaticCell<PacketQueue<TX_QUEUE_SIZE>> = StaticCell::new();

// TODO: Network task - runs the embassy-net stack
// #[embassy_executor::task]
// async fn net_task(stack: &'static Stack<Ethernet<'static, ETH>>) -> ! {
//     stack.run().await
// }

// TODO: Sensor reading task
// #[embassy_executor::task]
// async fn sensor_task(stack: &'static Stack<Ethernet<'static, ETH>>) {
//     // Wait for network to be ready
//     stack.wait_config_up().await;
//     info!("Network is up!");
//
//     loop {
//         // Simulate sensor reading
//         let temperature = read_sensor().await;
//
//         // Send data to server
//         if let Err(e) = send_sensor_data(stack, temperature).await {
//             warn!("Failed to send data: {:?}", e);
//         }
//
//         Timer::after(Duration::from_secs(10)).await;
//     }
// }

// TODO: Implement sensor reading (simulated for now)
// async fn read_sensor() -> i32 {
//     // In a real application, this would read from an I2C sensor
//     // For now, return a simulated temperature
//     static mut COUNTER: i32 = 20;
//     unsafe {
//         COUNTER = (COUNTER + 1) % 10 + 20;
//         COUNTER
//     }
// }

// TODO: Send sensor data to backend server via TCP
// async fn send_sensor_data(
//     stack: &'static Stack<Ethernet<'static, ETH>>,
//     temperature: i32,
// ) -> Result<(), embassy_net::tcp::Error> {
//     use embassy_net::tcp::TcpSocket;
//     use embedded_io_async::Write;
//     use core::fmt::Write as _;
//
//     let mut rx_buffer = [0; 1024];
//     let mut tx_buffer = [0; 1024];
//     let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
//
//     // TODO: Connect to server
//     // let remote_endpoint = (SERVER_ADDRESS.into(), SERVER_PORT);
//     // info!("Connecting to {:?}", remote_endpoint);
//     // socket.connect(remote_endpoint).await?;
//
//     // TODO: Format and send data
//     // let mut buffer = heapless::String::<128>::new();
//     // write!(&mut buffer, "{{\"temperature\": {}}}\n", temperature).ok();
//     // socket.write_all(buffer.as_bytes()).await?;
//     // info!("Sent: {}", buffer);
//
//     // TODO: Read response (optional)
//     // let mut response = [0; 128];
//     // let n = socket.read(&mut response).await?;
//     // info!("Response: {:a}", &response[..n]);
//
//     socket.close();
//     Ok(())
// }

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Session 13: Networking with embassy-net");

    // Configure system clock for 250MHz
    // This is necessary for Ethernet to work properly
    let mut config = StmConfig::default();
    config.rcc.hse = Some(Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: HseMode::Bypass,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: 2,
        mul: 125,
        divp: Some(2),
        divq: Some(2),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV2;
    config.rcc.apb3_pre = APBPrescaler::DIV2;
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.voltage_scale = VoltageScale::Scale0;

    let p = embassy_stm32::init(config);

    info!("Initializing Ethernet...");

    // TODO: Initialize Ethernet peripheral
    // Reference the STM32H5 Nucleo board pinout:
    // - RMII pins are typically on specific GPIO pins
    // - Consult your board documentation for exact pins

    // Example skeleton (pins need to be adjusted for your board):
    // let eth = Ethernet::new(
    //     RX_QUEUE.init(PacketQueue::new()),
    //     TX_QUEUE.init(PacketQueue::new()),
    //     p.ETH,
    //     Irqs,
    //     p.PA1,  // RMII REF CLK
    //     p.PA2,  // RMII MDIO
    //     p.PC1,  // RMII MDC
    //     p.PA7,  // RMII CRS DV
    //     p.PC4,  // RMII RXD0
    //     p.PC5,  // RMII RXD1
    //     p.PG11, // RMII TX EN
    //     p.PG13, // RMII TXD0
    //     p.PB13, // RMII TXD1
    //     GenericSMI::new(0), // PHY address
    //     [0x00, 0x80, 0xE1, 0x00, 0x00, 0x00], // MAC address
    // );

    info!("TODO: Initialize Ethernet peripheral with correct pins");
    info!("TODO: Create network stack with DHCP configuration");
    info!("TODO: Spawn network task");
    info!("TODO: Spawn sensor task");
    info!("TODO: Implement TCP client to send sensor readings");

    // Placeholder main loop
    loop {
        info!("Waiting for Ethernet and network implementation...");
        Timer::after(Duration::from_secs(5)).await;
    }
}
