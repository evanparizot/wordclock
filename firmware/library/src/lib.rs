#![no_std]

use cortex_m::asm;
use ds323x::{ic::DS3231, interface::I2cInterface, Ds323x, NaiveDate, DateTimeAccess, NaiveDateTime};
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
    gpio::{Alternate, Gpiob, OpenDrain, Pin, U},
    i2c::I2c,
    pac::{I2C1, SPI2},
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
    rtc: Ds323x<
        I2cInterface<
            I2c<
                I2C1,
                (
                    Pin<Gpiob, U<6>, Alternate<OpenDrain, 4>>,
                    Pin<Gpiob, U<7>, Alternate<OpenDrain, 4>>,
                ),
            >,
        >,
        DS3231,
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

    pub fn now(&mut self) -> NaiveDateTime {
        self.rtc.datetime().unwrap()
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
        .pclk2(32.MHz())
        .freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    //  /$$$$$$  /$$$$$$   /$$$$$$
    // |_  $$_/ /$$__  $$ /$$__  $$
    //   | $$  |__/  \ $$| $$  \__/
    //   | $$    /$$$$$$/| $$
    //   | $$   /$$____/ | $$
    //   | $$  | $$      | $$    $$
    //  /$$$$$$| $$$$$$$$|  $$$$$$/
    // |______/|________/ \______/
    //
    // DS3231
    // Default time is 01/01/00T00:00:00
    // https://github.com/eldruin/driver-examples/blob/master/stm32f3-discovery/examples/ds3231-f3.rs

    let mut scl =
        gpiob
            .pb6
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 100_000.Hz(), clocks, &mut rcc.apb1);
    let mut rtc = Ds323x::new_ds3231(i2c);

    //   /$$$$$$  /$$$$$$$  /$$$$$$
    //  /$$__  $$| $$__  $$|_  $$_/
    // | $$  \__/| $$  \ $$  | $$
    // |  $$$$$$ | $$$$$$$/  | $$
    //  \____  $$| $$____/   | $$
    //  /$$  \ $$| $$        | $$
    // |  $$$$$$/| $$       /$$$$$$
    //  \______/ |__/      |______/
    //
    // https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/spi.rs
    // https://github.com/almindor/max7219-examples/blob/master/examples/display_spi.rs
    // https://github.com/almindor/max7219

    let cs = gpiob
        .pb12
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
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
        rtc: rtc,
        delay: delay,
    }
}
