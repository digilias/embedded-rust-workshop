// Session 13 Snippet: Ethernet and Network Stack Initialization

use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources, Config};
use embassy_stm32::eth::{Ethernet, PacketQueue, generic_smi::GenericSMI};
use embassy_stm32::{bind_interrupts, eth, peripherals};
use static_cell::StaticCell;

const RX_QUEUE_SIZE: usize = 4;
const TX_QUEUE_SIZE: usize = 4;

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
});

static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
static RX_QUEUE: StaticCell<PacketQueue<RX_QUEUE_SIZE>> = StaticCell::new();
static TX_QUEUE: StaticCell<PacketQueue<TX_QUEUE_SIZE>> = StaticCell::new();

pub fn init_network(
    p: embassy_stm32::Peripherals,
    spawner: &Spawner,
) -> &'static Stack<Ethernet<'static, peripherals::ETH>> {
    // Initialize Ethernet peripheral
    // Note: Pin configuration depends on your board
    let eth = Ethernet::new(
        RX_QUEUE.init(PacketQueue::new()),
        TX_QUEUE.init(PacketQueue::new()),
        p.ETH,
        Irqs,
        p.PA1,  // RMII_REF_CLK
        p.PA2,  // RMII_MDIO
        p.PC1,  // RMII_MDC
        p.PA7,  // RMII_CRS_DV
        p.PC4,  // RMII_RXD0
        p.PC5,  // RMII_RXD1
        p.PG11, // RMII_TX_EN
        p.PG13, // RMII_TXD0
        p.PB13, // RMII_TXD1
        GenericSMI::new(0),  // PHY address
        [0x00, 0x80, 0xE1, 0x00, 0x00, 0x00], // MAC address
    );

    // Configure network stack with DHCP
    let config = Config::dhcpv4(Default::default());

    // Create network stack
    let stack = Stack::new(
        eth,
        config,
        RESOURCES.init(StackResources::new()),
        embassy_stm32::rng::Rng::new(p.RNG, Irqs).next_u64(),
    );

    // Leak to get 'static reference
    let stack = Box::leak(Box::new(stack));

    // Spawn network task
    spawner.spawn(net_task(stack)).unwrap();

    stack
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Ethernet<'static, peripherals::ETH>>) -> ! {
    stack.run().await
}

// Wait for network to be configured
pub async fn wait_for_network(stack: &Stack<impl embassy_net::driver::Driver>) {
    info!("Waiting for network...");
    stack.wait_config_up().await;

    if let Some(config) = stack.config_v4() {
        info!("Network configured:");
        info!("  IP: {:?}", config.address);
        info!("  Gateway: {:?}", config.gateway);
    }
}
