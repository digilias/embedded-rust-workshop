#![no_std]
#![no_main]

use embassy_stm32::i2c;
use embassy_time::Duration;
use embedded_hal::i2c::I2c;
use embassy_stm32::interrupt;
use embassy_stm32::pac;

const ADDRESS: u8 = 0x18;

use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut executor = SimpleExecutor::new();
    executor.block_on(async {
        let p = embassy_stm32::init(Default::default());

        setup();

        let mut config = i2c::Config::default();
        config.timeout = Duration::from_secs(2);
        let i2c = i2c::I2c::new_blocking(
            p.I2C1,
            p.PB8,
            p.PB9,
            config,
        );

        let mut driver = Lis3dh::new(I2cInterface::new(i2c));

        // Use it to read the register
        match driver.who_am_i().read() {
            Ok(whoami) => defmt::info!("Whoami: {:?}", whoami.value()),
            Err(e) => defmt::error!("Driver Error: {:?}", e),
        }

        loop {
            let mut data = [0u8; 6];
            let i2c = &mut driver.interface().i2c;
            match i2c.write_read(ADDRESS, &[0x28 | 0x80], &mut data) {
                Ok(_) => {
                    defmt::info!("{:?}", data);
                }
                Err(e) => {
                    defmt::error!("Error reading data: {:?}", e);
                }
            }
        }
    });
    loop {}
}




struct I2cInterface<I: I2c> {
    i2c: I,
}

impl<I: I2c> I2cInterface<I> {
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }
}

impl<I: I2c> device_driver::RegisterInterface for I2cInterface<I> {
    type Error = I::Error;
    type AddressType = u8;

    fn read_register(
        &mut self,
        address: Self::AddressType,
        size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        let size_bytes = (size_bits + 7) / 8;
        self.i2c
            .write_read(ADDRESS, &[address], &mut data[..size_bytes as usize])
    }

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        self.i2c.write(ADDRESS, &[address, data[0]])
    }
}

device_driver::create_device!(
    device_name: Lis3dh,
    dsl: {
        config {
            type RegisterAddressType = u8;
        }

        register WHO_AM_I {
            type Access = ReadWrite;
            const ADDRESS = 0x0F;
            const SIZE_BITS = 8;

            value: uint = 0..8,
        },

        register OUT_X_L {
            type Access = ReadOnly;
            const ADDRESS = 0x28;
            const SIZE_BITS = 8;

            value: uint = 0..8
        }
    }
);

fn setup() {
    let pin = 13;
    let port = 2;
    let rising = true;
    let falling = false;

    use pac::EXTI;
    EXTI.exticr(pin / 4).modify(|w| w.set_exti(pin % 4, port));
    EXTI.rtsr(0).modify(|w| w.set_line(pin, rising));
    EXTI.ftsr(0).modify(|w| w.set_line(pin, falling));
    EXTI.rpr(0).write(|w| w.set_line(pin, true));
    EXTI.fpr(0).write(|w| w.set_line(pin, true));
    EXTI.imr(0).modify(|w| w.set_line(pin, true));
}

#[interrupt]
unsafe fn EXTI13() {
    use pac::exti::regs::Lines;
    use pac::EXTI;

    let bits = EXTI.rpr(0).read().0 | EXTI.fpr(0).read().0;
    let bits = bits & 0x0000FFFF;
    EXTI.imr(0).modify(|w| w.0 &= !bits);

    EXTI.rpr(0).write_value(Lines(bits));
    EXTI.fpr(0).write_value(Lines(bits));

    let pin = 13;
    let imr = EXTI.imr(0).read();
    if !imr.line(pin) {
        defmt::info!("Hello!");
    }
    EXTI.imr(0).modify(|w| w.set_line(pin, true));
}

// Session 6 Snippet: Simple Executor Implementation
// Educational example - not for production use!

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub struct SimpleExecutor {
    // In a real executor, you'd store tasks in a queue
    // For simplicity, this example assumes tasks are externally managed
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {}
    }

    /// Run a single future to completion
    pub fn block_on<F: Future>(&mut self, mut future: F) -> F::Output {
        // Pin the future
        let mut future = unsafe { Pin::new_unchecked(&mut future) };

        // Create a dummy waker
        let waker = dummy_waker();
        let mut context = Context::from_waker(&waker);

        // Poll until ready
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    // In a real executor, we'd sleep or wait for events
                    // For this simple version, we just loop (busy-wait)
                }
            }
        }
    }
}

// Dummy waker implementation
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);

    RawWaker::new(core::ptr::null(), &VTABLE)
}
