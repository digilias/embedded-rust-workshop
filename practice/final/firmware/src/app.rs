use crate::{net, xl};
use static_cell::StaticCell;
use core::net::{SocketAddr, Ipv4Addr, IpAddr};
use embedded_nal_async::TcpConnect as _;
use defmt::*;

pub struct App {
    tcp: net::Client,
    stream: xl::SampleStream,
}

pub fn init(stream: xl::SampleStream, net: net::Net) -> App {
    static CLIENT_STATE: StaticCell<net::ClientState> = StaticCell::new();
    let state = CLIENT_STATE.init(net::ClientState::new());
    let tcp = state.bind(net);

    App {
        tcp,
        stream
    }

}


#[embassy_executor::task]
pub async fn run(app: App) {
    let App {
        tcp,
        stream
    } = app;
    let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 8080);
    loop {
        match tcp.connect(remote).await {
            Ok(connection) => {
                info!("Connected to {:?}. Forwarding stream...", remote);
                forward(stream, connection).await;
            }
            Err(e) => {
                warn!("Failed connecting to {:?}: {:?}", remote, e);
            }
        }
    }
}

async fn forward(stream: xl::SampleStream, _connection: net::Connection<'_>) {
    loop {
        let sample = stream.receive().await;
        defmt::info!("Forwarding sample: {:?}", sample);
    }
}
