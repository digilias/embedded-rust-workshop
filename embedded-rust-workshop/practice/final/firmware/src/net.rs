use crate::board::{NetResources, Irqs};

use defmt::info;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Stack, StackResources};
use embassy_stm32::eth::{Ethernet, GenericPhy, PacketQueue};
use embassy_stm32::peripherals::ETH;
use embassy_stm32::rng::Rng;
use static_cell::StaticCell;

type Device = Ethernet<'static, ETH, GenericPhy>;

pub struct Net {
    stack: Stack<'static>,
}

impl Net {
    pub fn socket<'a>(&self, rx: &'a mut [u8], tx: &'a mut [u8]) -> TcpSocket<'a> {
        let mut socket = TcpSocket::new(self.stack, rx, tx);
        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));
        socket
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
        p.pa2,
        p.pc1,
        p.pa7,
        p.pc4,
        p.pc5,
        p.pg13,
        p.pb15,
        p.pg11,
        GenericPhy::new_auto(),
        mac_addr,
    );

    // Generate random seed.
    let mut rng = Rng::new(p.rng, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let config = embassy_net::Config::dhcpv4(Default::default());
    //let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 61), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(10, 42, 0, 1)),
    //});

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    spawner.spawn(net_task(runner)).unwrap();

    // Ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;

    info!("Network task initialized");

    Net { stack }
}
