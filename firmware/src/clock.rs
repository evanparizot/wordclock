extern crate alloc;
use alloc::boxed::Box;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use ds323x::{ic::DS3231, interface::I2cInterface, DateTimeAccess, Ds323x, Timelike};
use hal::{
    gpio::{Alternate, Gpiob, OpenDrain, Output, Pin, PushPull, U},
    i2c::I2c,
    pac::{I2C1, SPI2},
    spi::Spi, delay::Delay,
};
use max7219::{connectors::SpiConnectorSW, MAX7219};

use crate::times::TimeMode;

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
    pub(crate) clock: Ds323x<
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
    pub(crate) mode: Box<dyn TimeMode + Send>,
}

impl Clock {
    pub fn update_display_time(&mut self) {
        // Get current time, in hours, minutes and seconds
        let (hour, minutes, _seconds) = self.time();
        let to_write = self.mode.get_time_arrays(hour, minutes);
        self.write_time_arrays(&to_write);
    }

    fn write_time_arrays(&mut self, time: &[[u8; 8]; 4]) {
        time.iter().enumerate().for_each(|(i, x)| {
            self.display.write_raw(i, x).unwrap();
        });
    }

    pub fn set_intensity(&mut self, displays: usize, intensity: u8) {
        for d in 0..displays {
            self.display.set_intensity(d, intensity).unwrap();
        }
    }

    pub fn wait(&mut self, ms: u16) {
        self.delay.delay_ms(ms);
    }

    pub fn add_minutes(&mut self) {
        // self.clock.set_minutes(minutes)
    }

    fn time(&mut self) -> (u32, u32, u32) {
        let datetime = self.clock.datetime().unwrap();
        let hour = datetime.time().hour();
        let minute = datetime.time().minute();
        let seconds = datetime.time().second();

        (hour, minute, seconds)
    }
}
