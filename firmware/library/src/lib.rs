#![no_std]

use cortex_m::asm;
use max7219::{connectors::SpiConnector, MAX7219};
pub use panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;

pub use stm32f3xx_hal::{
    delay::Delay,
    gpio::{gpioe, Output, PushPull},
    hal::blocking::delay::DelayMs,
    pac,
    prelude::*,
    spi::Spi,
};
use stm32f3xx_hal::{
    gpio::{Alternate, Gpiob, Pin, U},
    pac::SPI2,
};

pub struct Clock {
    display: MAX7219<
        SpiConnector<
            Spi<
                SPI2,
                (
                    Pin<Gpiob, U<13>, Alternate<PushPull, 5>>,
                    Pin<Gpiob, U<14>, Alternate<PushPull, 5>>,
                    Pin<Gpiob, U<15>, Alternate<PushPull, 5>>,
                ),
            >,
        >,
    >,
    delay: Delay,
}

impl Clock {
    pub fn write(&mut self) {
        // self.display.write_str(0, b"12345678", 0b0010_0000).unwrap();
    }

    pub fn on(&mut self) {
        self.display.test(0, true).unwrap();
    }

    pub fn off(&mut self) {
        self.display.test(0, false).unwrap();
    }

    pub fn pause(&mut self, ms: u16) {
        self.delay.delay_ms(ms);
    }
}

pub fn init() -> Clock {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let clocks = rcc
        .cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr);

    // let clocks = rcc
    //     .cfgr
    //     .use_hse(8.MHz())
    //     .sysclk(48.MHz())
    //     .pclk1(24.MHz())
    //     .freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    // https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/spi.rs
    // https://github.com/almindor/max7219-examples/blob/master/examples/display_spi.rs
    // https://github.com/almindor/max7219

    let sck = gpiob
        .pb13
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
    let miso = gpiob
        .pb14
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
    let mosi = gpiob
        .pb15
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);

    let spi = Spi::new(dp.SPI2, (sck, miso, mosi), 3.MHz(), clocks, &mut rcc.apb1);

    let mut display = MAX7219::from_spi(1, spi).unwrap();

    display.power_on().unwrap();
    display.clear_display(0).unwrap();

    Clock {
        display: display,
        delay: delay,
    }
}
