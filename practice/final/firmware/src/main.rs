#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};
use defmt::*;

mod xl;
mod net;
mod app;
mod board;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = board::init();

    let stream = unwrap!(xl::init(board.xl, spawner).await);
    let net = net::init(board.net, &spawner).await;
    let app = app::init(stream, net);

    spawner.spawn(unwrap!(app::run(app)));
}
