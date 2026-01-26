use cgmath::Vector3;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod renderer;
use renderer::{Renderer, Shape, ShapeInstance};

#[derive(Debug, Clone)]
struct ClientRotation {
    x: f32,
    y: f32,
    z: f32,
}

type ClientData = Arc<RwLock<HashMap<std::net::IpAddr, (Shape, ClientRotation)>>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let test_mode = std::env::args().any(|arg| arg == "--test");

    // Shared data between TCP server and renderer
    let clients: ClientData = Arc::new(RwLock::new(HashMap::new()));

    if test_mode {
        log::info!("Running in test mode with 10 simulated clients");

        // Create 10 test clients with different shapes
        {
            let mut clients_guard = clients.write().await;
            let shapes = [Shape::Cube, Shape::Pyramid, Shape::Torus, Shape::Cylinder];

            for i in 0..10 {
                // Create fake IP addresses: 192.168.0.1 through 192.168.0.10
                let ip: std::net::IpAddr = format!("192.168.0.{}", i + 1).parse().unwrap();
                let shape = shapes[i % shapes.len()].clone();
                clients_guard.insert(ip, (shape, ClientRotation { x: 0.0, y: 0.0, z: 0.0 }));
            }
            log::info!("Created {} test clients", clients_guard.len());
        }

        // Spawn task to slowly rotate all test shapes
        let clients_rotate = clients.clone();
        tokio::spawn(async move {
            let mut angle: f32 = 0.0;
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await; // ~60fps
                angle += 0.02; // Slow rotation speed

                let mut clients_guard = clients_rotate.write().await;
                for (_ip, (_shape, rotation)) in clients_guard.iter_mut() {
                    rotation.y = angle;
                }
            }
        });
    } else {
        // Spawn TCP server in normal mode
        let clients_tcp = clients.clone();
        tokio::spawn(async move {
            tcp_server(clients_tcp).await;
        });
    }

    // Run the rendering loop
    run_renderer(clients).await;
}

async fn tcp_server(clients: ClientData) {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind TCP listener");

    log::info!("TCP server listening on 0.0.0.0:8080");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let clients = clients.clone();
                tokio::spawn(async move {
                    handle_client(stream, addr, clients).await;
                });
            }
            Err(e) => {
                log::error!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_client(mut stream: TcpStream, addr: SocketAddr, clients: ClientData) {
    log::info!("New connection from: {}", addr);

    // Use IP address for consistent client identification across reconnections
    let client_ip = addr.ip();

    // Get or create shape for this IP
    {
        let mut clients_guard = clients.write().await;

        if let Some((shape, _)) = clients_guard.get(&client_ip) {
            // Reuse existing shape for this IP
            log::info!("Client {} reconnected (IP: {}), reusing existing shape: {:?}", addr, client_ip, shape);
            log::info!("Total clients in HashMap: {} - IPs: {:?}",
                clients_guard.len(),
                clients_guard.keys().collect::<Vec<_>>());
        } else {
            // Create new random shape for new IP
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let shape = match rng.gen_range(0..4) {
                0 => Shape::Cube,
                1 => Shape::Pyramid,
                2 => Shape::Torus,
                _ => Shape::Cylinder,
            };

            clients_guard.insert(
                client_ip,
                (shape.clone(), ClientRotation { x: 0.0, y: 0.0, z: 0.0 })
            );
            log::info!("Client {} assigned new shape: {:?}", addr, shape);
            log::info!("Total clients in HashMap: {} - IPs: {:?}",
                clients_guard.len(),
                clients_guard.keys().collect::<Vec<_>>());
        }
    }

    // Buffer for reading 3 f32 values (12 bytes total)
    let mut buffer = [0u8; 12];

    loop {
        match stream.read_exact(&mut buffer).await {
            Ok(_) => {
                // Parse 3 little-endian f32 values
                let x = f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let y = f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                let z = f32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

                // Check for invalid values
                if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                    log::warn!("Client {} sent invalid rotation: ({}, {}, {}) - raw bytes: {:02x?}",
                        addr, x, y, z, &buffer);
                    continue;
                }

                // Update rotation
                let mut clients_guard = clients.write().await;
                if let Some(client) = clients_guard.get_mut(&client_ip) {
                    client.1 = ClientRotation { x, y, z };
                }

                log::trace!("Updated rotation for {}: ({:.3}, {:.3}, {:.3})", addr, x, y, z);
            }
            Err(e) => {
                log::info!("Client {} disconnected: {}", addr, e);
                // We keep the client data even after disconnection
                break;
            }
        }
    }
}

async fn run_renderer(clients: ClientData) {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = Arc::new(
        event_loop.create_window(
            winit::window::Window::default_attributes()
                .with_title("TCP-Controlled 3D Shapes")
        ).expect("Failed to create window")
    );

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    event_loop
        .run(move |event, target| {
            target.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        // Update instances based on client data
                        let instances = {
                            if let Ok(clients_guard) = clients.try_read() {
                                let num_clients = clients_guard.len();
                                log::trace!("Rendering {} clients", num_clients);

                                // Calculate scale based on number of sensors
                                // More sensors = smaller objects to fit in view
                                let base_scale = 5.0; // Larger base scale to fill viewport better
                                let scale = if num_clients == 0 {
                                    base_scale
                                } else {
                                    (base_scale / (num_clients as f32).sqrt()).max(0.5).min(base_scale)
                                };

                                // Sort by IP address to ensure stable iteration order
                                let mut sorted_clients: Vec<_> = clients_guard.iter().collect();
                                sorted_clients.sort_by_key(|(ip, _)| *ip);

                                sorted_clients
                                    .iter()
                                    .enumerate()
                                    .map(|(index, (ip, (shape, rotation)))| {
                                        // Calculate grid dimensions for 16:10 aspect ratio
                                        let aspect_ratio = 16.0 / 10.0;
                                        let rows = ((num_clients as f32) / aspect_ratio).sqrt().ceil() as i32;
                                        let cols = ((num_clients as f32) / rows as f32).ceil() as i32;

                                        let row = (index as i32) / cols;
                                        let col = (index as i32) % cols;

                                        let spacing_x = 4.0;
                                        let spacing_y = 4.0;
                                        let x = (col as f32 - (cols - 1) as f32 / 2.0) * spacing_x;
                                        let y = ((rows - 1) as f32 / 2.0 - row as f32) * spacing_y;

                                        log::trace!("Instance {}: IP={}, shape={:?}, pos=({:.1},{:.1},{:.1}), rot=({:.2},{:.2},{:.2})",
                                            index, ip, shape, x, y, 0.0, rotation.x, rotation.y, rotation.z);

                                        ShapeInstance {
                                            shape: shape.clone(),
                                            position: Vector3::new(x, y, 0.0),
                                            rotation: Vector3::new(rotation.x, rotation.y, rotation.z),
                                            scale,
                                        }
                                    })
                                    .collect()
                            } else {
                                // If we can't acquire the lock, skip this frame
                                vec![]
                            }
                        };

                        renderer.update(instances);
                        
                        match renderer.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(e) => eprintln!("Render error: {:?}", e),
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .expect("Event loop failed");
}
