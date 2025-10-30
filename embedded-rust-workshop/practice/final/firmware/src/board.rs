use embassy_stm32::{bind_interrupts, eth, i2c, peripherals, rng, Config, Peri};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale,
};
use embassy_stm32::time::Hertz;
use assign_resources::assign_resources;

assign_resources! {
    net: NetResources {
        eth: ETH,
        pa1: PA1,
        pa2: PA2,
        pc1: PC1,
        pa7: PA7,
        pc4: PC4,
        pc5: PC5,
        pg13: PG13,
        pb15: PB15,
        pg11: PG11,
        rng: RNG,
    }
    xl: XlResources {
        i2c1: I2C1,
        scl: PB8,
        sda: PB9,
        dma1: GPDMA1_CH4,
        dma2: GPDMA1_CH5,
    }
}

pub struct Board {
    pub net: NetResources,
    pub xl: XlResources,
}

bind_interrupts!(pub struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    ETH => eth::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

pub fn init() -> Board {
    let mut config = Config::default();
    config.rcc.hsi = None;
    config.rcc.hsi48 = Some(Default::default());
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::BypassDigital,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL125,
        divp: Some(PllDiv::DIV2),
        divq: Some(PllDiv::DIV2),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.apb3_pre = APBPrescaler::DIV1;
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.voltage_scale = VoltageScale::Scale0;
    let p = embassy_stm32::init(config);

    let r = split_resources!(p);

    Board {
        net: r.net,
        xl: r.xl,
    }
}
