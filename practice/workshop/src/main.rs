#![no_std]
#![no_main]

use embassy_stm32::i2c::{Config, Error, I2c as I2cstm};
use embassy_stm32::pac;
use embassy_stm32::{
    bind_interrupts, exti,
    gpio::{Input, Pull},
    i2c, interrupt, peripherals,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    //#[embassy_executor::main]
    //async fn main(spawner: embassy_executor::Spawner) {

    // Initialize HAL
    let p = embassy_stm32::init(Default::default());

    // Create an i2c instance
    let mut config = Config::default();
    config.timeout = Duration::from_secs(2);
    let i2c = I2cstm::new_blocking(p.I2C1, p.PB8, p.PB9, config);

    let mut device = Lis3dh::new(I2cInterface::new(i2c));

    let val = device.who_am_i().read().unwrap().value();
    defmt::info!("whoami: {}", val);

    let mut button = Input::new(p.PC13, Pull::Down);
    // Pin: 13 port 2 rising false falling true drop true
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

    loop {}
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

use embedded_hal::i2c::I2c;

const ADDRESS: u8 = 0x18;

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
    }
);
