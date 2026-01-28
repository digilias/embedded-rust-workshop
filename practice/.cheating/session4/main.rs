#![no_std]
#![no_main]

use embassy_stm32::i2c;
use embassy_time::Duration;
use embedded_hal::i2c::I2c;

const ADDRESS: u8 = 0x18;

use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

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

    let mut i = 0;
    while i < 10 {
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
        i += 1;
    }
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
