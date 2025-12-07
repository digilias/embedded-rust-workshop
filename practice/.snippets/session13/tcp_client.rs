// Session 13 Snippet: TCP Client to Send Sensor Data

use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use defmt::{info, warn};

#[derive(Clone, Copy)]
pub struct Sample {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub async fn send_sample_to_server(
    stack: &Stack<'static>,
    sample: Sample,
    server_addr: (u8, u8, u8, u8),
    server_port: u16,
) -> Result<(), embassy_net::tcp::Error> {
    // Create socket buffers
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    // Set timeout
    socket.set_timeout(Some(Duration::from_secs(10)));

    // Connect to server
    let remote_endpoint = (server_addr.into(), server_port);
    info!("Connecting to {:?}", remote_endpoint);

    socket.connect(remote_endpoint).await?;
    info!("Connected!");

    // Format data as JSON
    let mut buffer = heapless::String::<128>::new();
    use core::fmt::Write as _;
    write!(
        &mut buffer,
        "{{\"x\":{:.2},\"y\":{:.2},\"z\":{:.2}}}\n",
        sample.x, sample.y, sample.z
    )
    .ok();

    // Send data
    socket.write_all(buffer.as_bytes()).await?;
    info!("Sent: {}", buffer);

    // Read response (optional)
    let mut response = [0; 128];
    match socket.read(&mut response).await {
        Ok(n) if n > 0 => {
            info!("Response: {:a}", &response[..n]);
        }
        Ok(_) => {}
        Err(e) => {
            warn!("Error reading response: {:?}", e);
        }
    }

    // Close connection
    socket.close();

    Ok(())
}

// Usage in main loop:
//
// loop {
//     let sample = sample_receiver.receive().await;
//
//     if let Err(e) = send_sample_to_server(
//         &stack,
//         sample,
//         (192, 168, 1, 100),  // Server IP
//         8080,                 // Server port
//     ).await {
//         warn!("Failed to send: {:?}", e);
//     }
//
//     Timer::after(Duration::from_secs(1)).await;
// }
