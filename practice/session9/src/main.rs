#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// TODO: Create a channel for sensor data
// static SENSOR_CHANNEL: Channel<NoopRawMutex, i32, 10> = Channel::new();

// TODO: Create a signal for commands
// static COMMAND_SIGNAL: Signal<NoopRawMutex, Command> = Signal::new();

// TODO: Create a mutex for shared state
// static SHARED_STATE: Mutex<NoopRawMutex, core::cell::RefCell<u32>> =
//     Mutex::new(core::cell::RefCell::new(0));

#[derive(Clone, Copy, defmt::Format)]
enum Command {
    Start,
    Stop,
    Reset,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Session 9: Synchronization Patterns");
    let _p = embassy_stm32::init(Default::default());

    // TODO: Spawn tasks
    // spawner.spawn(producer()).unwrap();
    // spawner.spawn(consumer()).unwrap();
    // spawner.spawn(command_sender()).unwrap();
    // spawner.spawn(command_receiver()).unwrap();

    info!("TODO: Implement synchronization examples");

    loop {
        Timer::after(Duration::from_secs(10)).await;
    }
}

// TODO: Implement producer task (sends data through channel)
// #[embassy_executor::task]
// async fn producer() {
//     let sender = SENSOR_CHANNEL.sender();
//     let mut value = 0;
//     loop {
//         sender.send(value).await;
//         info!("Produced: {}", value);
//         value += 1;
//         Timer::after(Duration::from_millis(500)).await;
//     }
// }

// TODO: Implement consumer task (receives from channel)
// #[embassy_executor::task]
// async fn consumer() {
//     let receiver = SENSOR_CHANNEL.receiver();
//     loop {
//         let value = receiver.receive().await;
//         info!("Consumed: {}", value);
//     }
// }

// TODO: Implement command sender (uses Signal)
// #[embassy_executor::task]
// async fn command_sender() {
//     loop {
//         Timer::after(Duration::from_secs(2)).await;
//         COMMAND_SIGNAL.signal(Command::Start);
//         info!("Sent command: Start");
//     }
// }

// TODO: Implement command receiver (waits on Signal)
// #[embassy_executor::task]
// async fn command_receiver() {
//     loop {
//         let cmd = COMMAND_SIGNAL.wait().await;
//         info!("Received command: {:?}", cmd);
//     }
// }
