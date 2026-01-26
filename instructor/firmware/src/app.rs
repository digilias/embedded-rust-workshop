use crate::{net, xl};
use static_cell::StaticCell;
use embedded_io_async::Write;
use core::net::{SocketAddr, Ipv4Addr, IpAddr};
use embedded_nal_async::TcpConnect as _;
use embassy_time::Timer;
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
    let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 8080);
    loop {
        match tcp.connect(remote).await {
            Ok(connection) => {
                info!("Connected to {:?}. Forwarding stream...", remote);

                if let Err(e) = forward(stream, connection).await {
                    warn!("Error while forwarding stream: {:?}", e);
                }
            }
            Err(e) => {
                warn!("Failed connecting to {:?}: {:?}", remote, e);
                Timer::after_secs(1).await;
            }
        }
    }
}

async fn forward(stream: xl::SampleStream, mut conn: net::Connection<'_>) -> Result<(), net::Error> {
    loop {
        let sample = stream.receive().await;
        info!("Forwarding sample: {:?}", sample);

        let (x, y, z) = (sample.x.to_le_bytes(), sample.y.to_le_bytes(), sample.z.to_le_bytes());
        conn.write_all(&x).await?;
        conn.write_all(&y).await?;
        conn.write_all(&z).await?;
    }
}
