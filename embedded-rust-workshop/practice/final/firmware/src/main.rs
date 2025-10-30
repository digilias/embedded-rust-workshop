#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};
use {defmt_rtt as _, panic_probe as _};

mod xl;
mod net;
mod board;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = board::init();

    let net = net::init(board.net, &spawner).await;
    let xl = xl::init(board.xl).await;

    loop {
        Timer::after(Duration::from_secs(1)).await;
        info!("Tick");
    }
}
