use ds323x::{ic::DS3231, interface::I2cInterface, DateTimeAccess, Ds323x, Timelike};
use max7219::{connectors::SpiConnectorSW, MAX7219};
use stm32f3xx_hal::{
    gpio::{Alternate, Gpiob, OpenDrain, Output, Pin, PushPull, U},
    i2c::I2c,
    pac::{I2C1, SPI2},
    spi::Spi,
};

use crate::times::{combine_phrases, FiveMinuteMode, Mode, ALL};

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
    pub(crate) mode: Mode,
}

impl Clock {
    pub fn update_time(&mut self) {
        // Get current time, in hours, minutes and seconds
        let (hour, minutes, _seconds) = self.time();

        match self.mode {
            Mode::FiveMinute => {
                // Map hour
                let hour_phrase = match (hour, minutes) {
                    (0 | 12, 31..=60) | (1 | 13, 0..=30) => FiveMinuteMode::ONE,
                    (1 | 13, 31..=60) | (2 | 14, 0..=30) => FiveMinuteMode::TWO,
                    (2 | 14, 31..=60) | (3 | 15, 0..=30) => FiveMinuteMode::THREE,
                    (3 | 15, 31..=60) | (4 | 16, 0..=30) => FiveMinuteMode::FOUR,
                    (4 | 16, 31..=60) | (5 | 17, 0..=30) => FiveMinuteMode::FIVE,
                    (5 | 17, 31..=60) | (6 | 18, 0..=30) => FiveMinuteMode::SIX,
                    (6 | 18, 31..=60) | (7 | 19, 0..=30) => FiveMinuteMode::SEVEN,
                    (7 | 19, 31..=60) | (8 | 20, 0..=30) => FiveMinuteMode::EIGHT,
                    (8 | 20, 31..=60) | (9 | 21, 0..=30) => FiveMinuteMode::NINE,
                    (9 | 21, 31..=60) | (10 | 22, 0..=30) => FiveMinuteMode::TEN,
                    (10 | 22, 31..=60) | (11 | 23, 0..=30) => FiveMinuteMode::ELEVEN,
                    (11 | 23, 31..=60) | (0 | 12, 0..=30) => FiveMinuteMode::TWELVE,
                    _ => ALL,
                };

                // Map minutes
                let minute_phrase = match minutes {
                    0..=4 => FiveMinuteMode::O_CLOCK,
                    5..=9 => FiveMinuteMode::FIVE_PAST,
                    10..=14 => FiveMinuteMode::TEN_PAST,
                    15..=19 => FiveMinuteMode::QUARTER_PAST,
                    20..=24 => FiveMinuteMode::TWENTY_PAST,
                    25..=29 => FiveMinuteMode::TWENTY_FIVE_PAST,
                    30..=34 => FiveMinuteMode::HALF_PAST,
                    35..=39 => FiveMinuteMode::TWENTY_FIVE_TO,
                    40..=44 => FiveMinuteMode::TWENTY_TO,
                    45..=49 => FiveMinuteMode::A_QUARTER_TO,
                    50..=54 => FiveMinuteMode::TEN_TO,
                    55..=59 => FiveMinuteMode::FIVE_TO,
                    _ => ALL,
                };

                let modifier_phrase = match hour {
                    7..=11 => FiveMinuteMode::IN_THE_MORNING,
                    12..=15 => FiveMinuteMode::IN_THE_AFTERNOON,
                    16..=20 => FiveMinuteMode::IN_THE_EVENING,
                    21..=23 | 0..=6 => FiveMinuteMode::AT_NIGHT,
                    _ => ALL,
                };

                let to_write = combine_phrases(&[
                    FiveMinuteMode::IT_IS,
                    hour_phrase,
                    minute_phrase,
                    modifier_phrase,
                ]);
                self.write_time(&to_write);
            }
            Mode::OneMinute => {}
        }
    }

    pub fn write_time(&mut self, time: &[[u8; 8]; 4]) {
        time.iter().enumerate().for_each(|(i, x)| {
            self.display.write_raw(i, x).unwrap();
        });
    }

    pub fn set_intensity(&mut self, displays: usize, intensity: u8) {
        for d in 0..displays {
            self.display.set_intensity(d, intensity).unwrap();
        }
    }

    fn time(&mut self) -> (u32, u32, u32) {
        let datetime = self.rtc.datetime().unwrap();
        let hour = datetime.time().hour();
        let minute = datetime.time().minute();
        let seconds = datetime.time().second();

        (hour, minute, seconds)
    }
}
