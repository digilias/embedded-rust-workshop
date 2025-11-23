# Session 13: Networking with embassy-net

This session demonstrates how to use embassy-net to send sensor data over Ethernet using TCP/IP.

## Goal

Learn to use embassy-net by:
- Initializing the Ethernet peripheral on STM32H5
- Setting up the network stack with DHCP
- Using the control + runner pattern
- Sending sensor data via TCP to a backend server

## What's Provided

- `src/main.rs`: Skeleton code with:
  - Clock configuration for STM32H5 (250MHz required for Ethernet)
  - Network buffer allocations
  - TODO markers for you to complete
  - Example structure for TCP client

## Your Tasks

### 1. Initialize Ethernet Peripheral
Consult your STM32H563 Nucleo board documentation to find the correct RMII pins:
- Reference Clock (REF_CLK)
- MDIO/MDC (management interface)
- CRS_DV (carrier sense/data valid)
- RXD0, RXD1 (receive data)
- TX_EN (transmit enable)
- TXD0, TXD1 (transmit data)

Initialize the Ethernet peripheral with these pins.

### 2. Create Network Stack
```rust
let config = Config::dhcpv4(Default::default());

let stack = Stack::new(
    eth,
    config,
    RESOURCES.init(StackResources::new()),
    embassy_stm32::rng::Rng::new(p.RNG).next_u64(),
);
```

Make `stack` static using `make_static!()` macro or `StaticCell`.

### 3. Spawn Network Task
The network task runs the stack's event loop:
```rust
spawner.spawn(net_task(&stack)).unwrap();
```

Uncomment and complete the `net_task` function.

### 4. Implement Sensor Task
- Wait for network to be configured (`stack.wait_config_up().await`)
- Periodically read sensor data (simulated)
- Send data to backend server via TCP

### 5. Implement TCP Client
Complete the `send_sensor_data` function:
- Create TCP socket with buffers
- Connect to server endpoint
- Format data (e.g., as JSON)
- Send data over socket
- Handle errors appropriately

### 6. Backend Server
Set up a simple TCP server on your development machine:

```python
# simple_server.py
import socket

HOST = '0.0.0.0'
PORT = 8080

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind((HOST, PORT))
    s.listen()
    print(f'Server listening on {HOST}:{PORT}')

    while True:
        conn, addr = s.accept()
        with conn:
            print(f'Connected by {addr}')
            data = conn.recv(1024)
            if data:
                print(f'Received: {data.decode()}')
                conn.sendall(b'OK\n')
```

Run: `python3 simple_server.py`

## Key Concepts

### Control + Runner Pattern
Embassy-net uses a split design:
- **Runner task**: Processes network stack internals (spawned once)
- **Control handle**: Used by application code to create sockets

This pattern provides:
- Clean separation of concerns
- Efficient single-task event processing
- Safe concurrent access to network stack

### Network Stack Lifecycle
```
1. Initialize Ethernet peripheral
2. Create Stack with configuration
3. Spawn runner task (runs forever)
4. Wait for configuration (DHCP/static)
5. Use stack to create sockets
```

### Socket Buffer Management
Each socket needs RX and TX buffers:
```rust
let mut rx_buffer = [0; 1024];
let mut tx_buffer = [0; 1024];
let socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
```

Buffers must live as long as the socket is in use.

## Building and Running

1. Connect Ethernet cable to your Nucleo board
2. Connect to the same network as your development machine
3. Start the backend server
4. Build and flash firmware:
```bash
cargo build
cargo run
```

5. Watch the logs:
```
INFO  Session 13: Networking with embassy-net
INFO  Initializing Ethernet...
INFO  Network is up!
INFO  IP: 192.168.1.123
INFO  Connecting to 192.168.1.100:8080
INFO  Sent: {"temperature": 23}
```

## Debugging Tips

### Check Link Status
```rust
if !stack.is_link_up() {
    warn!("Ethernet link is down - check cable");
}
```

### Check IP Configuration
```rust
if let Some(config) = stack.config_v4() {
    info!("IP: {:?}", config.address);
    info!("Gateway: {:?}", config.gateway);
}
```

### Enable Detailed Logging
In `.cargo/config.toml`:
```toml
[env]
DEFMT_LOG = "trace"
```

## Extension Challenges

1. **HTTP Server**: Implement a simple HTTP endpoint that returns sensor data
2. **JSON Formatting**: Use `serde-json-core` for proper JSON serialization
3. **UDP Discovery**: Implement UDP broadcast for device discovery
4. **Retry Logic**: Add exponential backoff for connection failures
5. **Multiple Sensors**: Send data from multiple sensors in one request
6. **REST API**: Create GET/POST endpoints for sensor configuration

## Common Issues

**Link won't come up**
- Check Ethernet cable
- Verify PHY address (usually 0 or 1)
- Check RMII pin configuration

**DHCP not working**
- Verify network has DHCP server
- Try static IP configuration instead
- Check firewall settings

**Can't connect to server**
- Verify server is running and reachable
- Check firewall on development machine
- Ping both directions to verify network connectivity
- Ensure server address and port match

**Socket errors**
- Check buffer sizes are adequate
- Verify stack.wait_config_up() is called before creating sockets
- Make sure network task is spawned
