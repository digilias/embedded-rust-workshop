// Session 9 Snippet: Using Signal for Event Notification

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use defmt::info;

#[derive(Clone, Copy, defmt::Format)]
pub enum Command {
    Start,
    Stop,
    Reset,
}

// Signal for sending commands
static COMMAND_SIGNAL: Signal<ThreadModeRawMutex, Command> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(command_sender()).unwrap();
    spawner.spawn(command_handler()).unwrap();
}

#[embassy_executor::task]
async fn command_sender() {
    loop {
        Timer::after(Duration::from_secs(1)).await;
        info!("[sender] Sending Start");
        COMMAND_SIGNAL.signal(Command::Start);

        Timer::after(Duration::from_secs(2)).await;
        info!("[sender] Sending Stop");
        COMMAND_SIGNAL.signal(Command::Stop);
    }
}

#[embassy_executor::task]
async fn command_handler() {
    loop {
        // Wait for command (blocks until signal is sent)
        let cmd = COMMAND_SIGNAL.wait().await;

        info!("[handler] Received: {:?}", cmd);

        match cmd {
            Command::Start => {
                info!("Starting...");
            }
            Command::Stop => {
                info!("Stopping...");
            }
            Command::Reset => {
                info!("Resetting...");
            }
        }
    }
}
