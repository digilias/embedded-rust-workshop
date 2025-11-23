---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 13: Networking with embassy-net

* **Goal:** Using the TCP/IP stack to send sensor data over Ethernet

---

# Network stack components

```
┌─────────────────────────────────┐
│   Application (TCP/UDP)         │
├─────────────────────────────────┤
│   embassy-net (async API)       │
├─────────────────────────────────┤
│   smoltcp (TCP/IP stack)        │
├─────────────────────────────────┤
│   Driver (Ethernet MAC)         │
└─────────────────────────────────┘
```

---


# `smoltcp`

* Pure Rust TCP/IP stack
* `no_std` compatible
* Supports TCP, UDP, ICMP, DHCPv4, IPv6
* Wire protocol implementations without OS dependencies
* Memory-efficient (configurable buffers)

---

# `embassy-net` overview

* Async TCP/IP network stack for embedded systems
* Built on top of `smoltcp`
* Designed for resource-constrained devices
* Integrates seamlessly with async rust

---

# Control + Runner pattern

* **Runner task**: Processes network stack events
* **Control handle**: Application interface to the stack

```rust
#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await  // Never returns
}

async fn main(spawner: Spawner) {
    let stack = Stack::new(/* ... */);
    spawner.spawn(net_task(stack)).unwrap();
    // Use stack via control handle
}
```

---

# Why this pattern?

* **Separation of concerns**
  - Runner handles internal state machine
  - Application uses clean async API
* **Efficiency**
  - Single task processes all network events
  - No polling overhead
* **Safety**
  - Stack internals are private
  - Only safe operations exposed

---

# Setting up embassy-net

```rust
use embassy_net::{Stack, StackResources, Config};

// Statically allocated resources
static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

let config = Config::dhcpv4(Default::default());

let stack = Stack::new(
    device,
    config,
    RESOURCES.init(StackResources::new()),
    seed,  // Random seed for stack
);
```

---

# Network configuration options

```rust
// DHCP (automatic)
let config = Config::dhcpv4(Default::default());

// Static IP
let config = Config::ipv4_static(StaticConfigV4 {
    address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 1, 100), 24),
    gateway: Some(Ipv4Address::new(192, 168, 1, 1)),
    dns_servers: Vec::new(),
});
```

---

# TCP client example

```rust
use embassy_net::tcp::TcpSocket;

let mut tx_buffer = [0u8; 2048];
let mut rx_buffer = [0u8; 2048];
let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

// Connect to server
socket.connect(remote_endpoint).await?;

// Write data
socket.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;

// Read response
let mut buffer = [0; 1024];
let n = socket.read(&mut buffer).await?;
```

---

# TCP server example

```rust
use embassy_net::tcp::TcpSocket;

let mut tx_buffer = [0u8; 2048];
let mut rx_buffer = [0u8; 2048];
let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

// Listen on port
socket.accept(8080).await?;

// Read request
let mut buffer = [0; 1024];
let n = socket.read(&mut buffer).await?;

// Send response
socket.write_all(b"HTTP/1.0 200 OK\r\n\r\n").await?;
socket.write_all(response_data).await?;
```

---

# DNS resolution

```rust
use embassy_net::dns::DnsQueryType;

let address = stack
    .dns_query("example.com", DnsQueryType::A)
    .await?;

info!("Resolved to: {:?}", address);
```

---

# Resource considerations

* **Buffer sizing**
  - TX/RX buffers per socket
  - Trade-off: memory vs throughput
* **Socket limits**
  - Defined in `StackResources<N>`
  - Each socket costs memory

---

# Exercise

* Ensure network cable connected to devkit
* Initialize Ethernet peripheral with embassy-stm32
* Configure embassy-net with DHCP
* Create TCP client to send xl data (3xf32 as little endian)
* Send accelerometer readings

Starting code in `practice/session13`
