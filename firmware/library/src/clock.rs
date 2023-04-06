use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use ds323x::{ic::DS3231, interface::I2cInterface, Ds323x, NaiveDateTime, DateTimeAccess, Timelike, NaiveTime};
use max7219::{connectors::SpiConnectorSW, MAX7219};
use stm32f3xx_hal::{
    gpio::{Alternate, Gpiob, OpenDrain, Output, Pin, PushPull, U},
    i2c::I2c,
    pac::{I2C1, SPI2},
    spi::Spi, delay::Delay,
};

pub struct Clock {
    pub(crate) display: MAX7219<
        SpiConnectorSW<
            Spi<
                SPI2,
                (
                    Pin<Gpiob, U<13>, Alternate<PushPull, 5>>,
                    Pin<Gpiob, U<14>, Alternate<PushPull, 5>>,
                    Pin<Gpiob, U<15>, Alternate<PushPull, 5>>,
                ),
            >,
            Pin<Gpiob, U<12>, Output<PushPull>>,
        >,
    >,
    pub(crate) rtc: Ds323x<
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
    pub(crate) delay: Delay,
    
}

impl Clock {
    pub fn write_str(&mut self, addr: usize, string: &[u8; 8], dots: u8) {
        self.display.write_str(addr, string, dots).unwrap();
    }

    pub fn write_raw(&mut self, addr: usize, raw: &[u8; 8]) {
        self.display.write_raw(addr, raw).unwrap();
    }

    pub fn on(&mut self, addr: usize) {
        self.display.test(addr, true).unwrap();
    }

    pub fn off(&mut self, addr: usize) {
        self.display.test(addr, false).unwrap();
    }

    pub fn clear(&mut self, addr: usize) {
        self.display.clear_display(addr).unwrap();
    }

    pub fn pause(&mut self, ms: u16) {
        self.delay.delay_ms(ms);
    }

    pub fn set_intensity(&mut self, addr: usize, intensity: u8) {
        self.display.set_intensity(addr, intensity).unwrap();
    }

    pub fn time(&mut self) -> (u32, u32, u32) {
        let datetime = self.rtc.datetime().unwrap();
        let hour = datetime.time().hour();
        let minute = datetime.time().minute();
        let seconds = datetime.time().second();

        (hour, minute, seconds)
    }

}
