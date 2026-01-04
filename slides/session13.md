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

# `embassy-net`

* Async TCP/IP network stack for embedded systems
* Built on top of `smoltcp`

---

# `embassy-net-driver` 

* Driver traits for implementing drivers that work with `embassy-net`
* Examples
  * `embassy-net-ppp`
  * `embassy-net-nrf91`
  * `embassy-net-wiznet`
  * `embassy-net-esp-hosted`
  * `embassy-net-en28j60`
  * `embassy-net-adin1110`
  * `embassy-net-tuntap`

--- 

# `embassy-net-driver-channel` 

* Helper crate for implementing drivers

---

# Control + Runner pattern

* **Runner task**: Drives the TCP/IP stack with the driver
* **Control handle**: Application interface to the stack

```rust
#[embassy_executor::task]
async fn net_task(runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await  // Never returns
}

async fn main(spawner: Spawner) {
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    
    let seed: [u8; 8] = [0; 8]; // chosen by fair dice roll. guaranteed to be random

    let (stack, runner) = embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);
    spawner.spawn(net_task(runner).unwrap());

    // Example: ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;

    // Now you can create TCP sockets
}
```

---

# TCP socket - client

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

# TCP socket - server

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

# Higher level protocols

* `embedded-nal-async` - implemented by embassy_net::tcp::client
* `reqwless` - http client
* `rust_mqtt` - mqtt client
* `embedded-tls` - rust TLS stack

---

# Resource considerations

* **Buffer sizing**
  - TX/RX buffer sizes
* **Socket limits**
  - Defined in `StackResources<N>`
  - Each socket uses RAM

---

# Exercise

* Connect network cable to devkit
* Initialize stm32h5 ethernet driver
  ```rust
      let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
      static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
      let device = Ethernet::new(
          PACKETS.init(PacketQueue::<4, 4>::new()),
          p.eth,
          Irqs,
          p.pa1, p.pa7, p.pc4, p.pc5, p.pg13, p.pb15, p.pg11, mac_addr, p.eth_sma, p.pa2, p.pc1,
      );
  ```
* Configure embassy-net 
* Create TCP socket
* Connect to IP X.X.X.X
* Send accelerometer data (3xf32 as little endian)
