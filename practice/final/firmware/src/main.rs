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

//    let net = net::init(board.net, &spawner).await;
    let mut xl = xl::init(board.xl).await;

    loop {
        let sample = xl.sample().await;
        defmt::info!("sample: {:?}", sample);
    }
}
