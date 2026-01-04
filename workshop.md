# Embedded Rust Workshop

We'll start from scratch with blinky, then build and improve an embedded application that:

* Reads data from an I2C sensor
* Logging data and events
* Uses async/await to take advantage of unique Rust features
* Demonstrates common patterns for composing Rust applications
* Sharing data over TCP/IP network

The idea is to evolve the application through the sessions, starting with basics and ending up with an full blown application. By the end of the workshop, participants should be able to: 

1. Set up and configure embedded rust projects for stm32
2. Understand the differences between the PAC, HAL, and applying the appropriate abstractions.
3. Writing platform-agnostic drivers using embedded-hal and device-driver crates
4. Understand how and when to use async in embedded
5. Integrate with C libraries when needed
6. Apply proper logging, error handling, and testing strategies
7. Writing network code with embassy-net


## Equipment we will use

* stm32h563zi nucleo boards (1 per participant)
* I2C accelerometer + wires
* Network switch + cables
* Laptop to represent a backend

## Prerequisites

Participants are expected to be familiar with these concepts:

* How to build using the cargo tool
* The basics of the Cargo.toml manifest
* Using structs, enums and traits

Participants can read up on and learn the above following these guides:

* https://rust-lang.org/learn/get-started/
* https://rustlings.rust-lang.org/
* https://doc.rust-lang.org/stable/book/

Participants are expected to have the following tools installed:

* rustup - https://rustup.rs/ - it's important to install rust using rustup because we need rustup for additional tools
* probe-rs https://probe.rs/ - remember to setup the correct permissions as described in https://probe.rs/docs/getting-started/probe-setup/
* ARM gcc toolchain (for building C library and various tools) for bare metal - https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads

Each session will contain a 5-20 minute theory part, and a practice part where participants will perform some exercises building up the application with assistance.

Participants should run through session 1 on their own to make sure they have everything working with the devkit beforehand:

* Clone the workshop repository: https://github.com/digilias/embedded-rust-workshop
* Follow the instructions for session 1: https://github.com/digilias/embedded-rust-workshop/blob/main/slides/session1.md

See you on January 29th. If there any questions or special requirements in advance, feel free to reach me at ulf@digili.no

Best regards,

Ulf

## Software

* Workshop repository with example skeleton code
* Example solutions for each session

## Documentation

* Embassy cheat sheets
* Common error solutions

## Sessions

Each session is separated by a short break (except the lunch break). We go through the sessions in order, so we cover as much as possible the first day, and continue on the second day.

Covering all of the topics might be too ambitious, but we can speed up or skip some sessions depending on feedback underway.

## Session 1: Foundation (Before workshop)

**Goal:** Setting up project and getting a simple blinky example to run. 

**Theory** Embedded Rust vs Regular Rust

* Memory constraints and #![no_std]
* Ownership and alloc in embedded
* Project structure and cargo 

**Practice:** Project setup and basic GPIO

* Set up a rust project for stm32h5
* Verify toolchain and probe setup
* Blink LED using direct register access (PAC)

## Session 2: PAC vs HAL

**Goal:** Introducing using peripherals with the Peripheral Access Crate (PAC) concept and using that to work with registers.

**Theory:** PAC vs HAL

* Register-level programming with stm32-metapac

**Practice:** I2C with HAL

* Configure I2C peripheral using stm32-metapac
* Implement basic I2C read/write operations
* Read device ID from I2C sensor

## Session 3: Platform-Agnostic Drivers with embedded-hal

**Goal:** Using embedded-hal + device-driver to create generic device drivers.

**Theory:** embedded-hal

* Traits for hardware abstraction
* Writing portable drivers
* Driver ecosystem (device-driver crate)

**Practice:** Generic I2C driver

* Refactor I2C code to use embedded-hal traits
* Create reusable sensor driver using device-driver crate
* Test driver with mock implementation

## Session 4: Using HAL for programming

**Goal:** Using embassy-stm32 as HAL for the device-driver application.

**Theory:** embassy-stm32

* High-level peripheral drivers
* Timers and delays

**Practice:** I2C with embassy-stm32

* Migrate application to embassy-stm32 HAL
* Add periodic sensor polling

## Session 5: Handling interrupts

**Goal:** How to setup interrupts for the i2c sensor and reacting to it.

**Theory:** How interrupts are defined and handled

* Introducing the macros from cortex-m

**Practice:** Handling sensor interrupts

* Using cortex-m-rt crate to define interrupt handlers

## Session 6: Async Fundamentals

**Goal:** Understanding how async works.

**Theory:** Understanding async

* Futures, tasks, and executors
* Async vs interrupts vs threads
* Building a minimal executor from scratch

**Practice:** Simple executor implementation

* Write basic executor from scratch
* Understand waker mechanics

## Session 7: Handling interrupts with async

**Goal:** Understand how to combine async with interrupts

**Theory:** Creating wakers and manual future implementations

**Practice:** Modifying application to use async API for sensor

## Session 8: Embassy executor

**Goal:*** Understanding the embassy executor

**Theory:** How the executor works

* Sleeping and waking
* Priorities and deadline
* System timer

**Practice:** Refactor application to use embassy-executor

* Implement async sensor polling task
* Add async delays and timeouts


## Session 9: Synchronization patterns

**Goal:** Introducing different ways to communicate between tasks

**Theory:** Data synchronization

* Mutexes, channels, and signals
* Critical sections and atomic operations

**Practice:** Task communication

* Add data sharing between tasks
* Implement producer-consumer with channels


## Session 10: Executor and preemption

**Goal**: Understanding how to use multiple executors

**Theory**: Using multiple executors

* NoopRawMutex vs ThreadRawMutex vs CriticalSectionRawMutex
* Send, Sync and how the compiler helps us make the correct choice

**Practice**: Defining a higher priority task running on another executor.


## Session 11: Panicking, logging and debugging

**Goal:** How to customize panic handlers and how logging works.

**Theory:** Panic/exception handler and defmt 

* How RTT works
* How defmt works

**Practice:** Creating a custom panic handler

* Defining a custom panic handler
* Save some state in panic handler
* Log panicked state in startup

## Session 12: Wrapping C

**Goals:** Wrapping an existing C library.

**Theory:** Unsafe & wrapping C

* When unsafe is necessary/appropriate
* Using bindgen for C library integration

**Practice:** C Library Integration

* Create bindings for a C math library
* Safely interface with C code
* Handle C-style error codes and memory management

## Session 13: Networking

**Goal:** Using the TCP/IP stack in embassy.

**Theory:** embassy-net and smoltcp

* TCP/IP stack overview
* Writing networking drivers
* Control + runner pattern

**Practice:** reporting sensor data to 'the cloud'

* Add support for networking with embassy-net
* Publish sensor data over TCP

## Additional sessions if time left

* Testing
* Embassy-boot and OTA
* Tuning/profiling async


TODO:

* Session 11: Summary in slide + debug slide. Practice code
* Session 12: Summary in slide + practice C library
* Session 13: Summary in slide + practice code.


* Sending out info in January:

* PDF with intro and first session1 - links to github
