// Session 9 Snippet: Producer-Consumer with Channel

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Sender, Receiver};
use embassy_time::{Duration, Timer};
use defmt::info;

#[derive(Clone, Copy, defmt::Format)]
pub struct Sample {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

// Static channel with 10-element buffer
static SAMPLE_CHANNEL: Channel<ThreadModeRawMutex, Sample, 10> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());

    // Get sender and receiver
    let sender = SAMPLE_CHANNEL.sender();
    let receiver = SAMPLE_CHANNEL.receiver();

    // Spawn producer and consumer
    spawner.spawn(producer(sender)).unwrap();
    spawner.spawn(consumer(receiver)).unwrap();
}

#[embassy_executor::task]
async fn producer(sender: Sender<'static, ThreadModeRawMutex, Sample, 10>) {
    let mut count = 0i16;

    loop {
        // Simulate reading accelerometer
        let sample = Sample {
            x: count,
            y: count + 1,
            z: count + 2,
        };

        info!("[producer] Sending: {:?}", sample);

        // Send sample (blocks if channel is full)
        sender.send(sample).await;

        count += 1;
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn consumer(receiver: Receiver<'static, ThreadModeRawMutex, Sample, 10>) {
    loop {
        // Receive sample (blocks if channel is empty)
        let sample = receiver.receive().await;

        info!("[consumer] Received: {:?}", sample);

        // Process sample (simulate slow processing)
        Timer::after(Duration::from_millis(200)).await;
    }
}
