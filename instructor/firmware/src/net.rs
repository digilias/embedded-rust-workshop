use crate::board::{NetResources, Irqs};

use defmt::info;
use embassy_executor::Spawner;
use embassy_net::tcp::{self, client::{TcpClient, TcpClientState, TcpConnection}};
use embassy_net::{Ipv4Cidr, Stack, StackResources};
use embassy_stm32::eth::{Ethernet, GenericPhy, PacketQueue, Sma};
use embassy_stm32::peripherals::{ETH_SMA, ETH};
use embassy_stm32::rng::Rng;
use heapless::Vec;
use core::net::Ipv4Addr;
use static_cell::StaticCell;

type Device = Ethernet<'static, ETH, GenericPhy<Sma<'static, ETH_SMA>>>;

pub struct Net {
    stack: Stack<'static>,
}

// We can make 2 TCP connections
pub type Client = TcpClient<'static, 2, 2048, 2048>;
pub type Connection<'d> = TcpConnection<'d, 2, 2048, 2048>;
pub type Error = tcp::Error;

pub struct ClientState {
    state: TcpClientState<2, 2048, 2048>,
}

impl ClientState {
    pub fn new() -> ClientState {
        Self {
            state: TcpClientState::new(),
        }
    }

    pub fn bind(&'static mut self, net: Net) -> Client {
        TcpClient::new(net.stack, &self.state)
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

pub async fn init(p: NetResources, spawner: &Spawner) -> Net {
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    
    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.eth,
        Irqs,
        p.pa1,
        p.pa7,
        p.pc4,
        p.pc5,
        p.pg13,
        p.pb15,
        p.pg11,
       mac_addr,
        p.eth_sma,
        p.pa2,
        p.pc1,
    );

    // Generate random seed.
    let mut rng = Rng::new(p.rng, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    // let config = embassy_net::Config::dhcpv4(Default::default());
    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Addr::new(10, 0, 0, 2), 24),
        dns_servers: Vec::new(),
        gateway: Some(Ipv4Addr::new(10, 0, 0, 1)),
    });

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    spawner.must_spawn(net_task(runner));

    // Ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;

    info!("Network task initialized");

    Net { stack }
}
