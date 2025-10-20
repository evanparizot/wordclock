use crate::times::{EMPTY, Phrase, ALL, TimeMode, combine_phrases};


pub struct DefaultMode {}

impl DefaultMode {
    pub const ONE: Phrase = [EMPTY, [0, 0, 0, 0, 0, 0, 0, 0b00000111], EMPTY, EMPTY];
    pub const TWO: Phrase = [EMPTY, [0, 0, 0, 0, 0, 0, 0, 0b11100000], EMPTY, EMPTY];
    pub const THREE: Phrase = [[0, 0, 0, 0, 0, 0, 0, 0b11111000], EMPTY, EMPTY, EMPTY];
    pub const FOUR: Phrase = [EMPTY, EMPTY, [0, 0b11110000, 0, 0, 0, 0, 0, 0], EMPTY];
    pub const FIVE: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0b00001111, 0, 0, 0, 0, 0, 0]];
    pub const SIX: Phrase = [EMPTY, EMPTY, [0, 0, 0b00111000, 0, 0, 0, 0, 0], EMPTY];
    pub const SEVEN: Phrase = [EMPTY, EMPTY, [0b00111110, 0, 0, 0, 0, 0, 0, 0], EMPTY];
    pub const EIGHT: Phrase = [EMPTY, EMPTY, EMPTY, [0b00111110, 0, 0, 0, 0, 0, 0, 0]];
    pub const NINE: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0b11110000, 0, 0, 0, 0, 0]];
    pub const TEN: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0, 0b00011100, 0, 0, 0, 0]];
    pub const ELEVEN: Phrase = [EMPTY, EMPTY, [0, 0, 0, 0b11111100, 0, 0, 0, 0], EMPTY];
    pub const TWELVE: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0b00000011, 0, 0, 0, 0, 0, 0],
        [0, 0b11110000, 0, 0, 0, 0, 0, 0],
    ];

    pub const FIVE_PAST: Phrase = [
        [0, 0, 0b00000001, 0, 0, 0b00001111, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const TEN_PAST: Phrase = [
        [0, 0, 0, 0, 0, 0b00001111, 0, 0],
        [0b00011100, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const A_QUARTER_PAST: Phrase = [
        [0, 0b10111111, 0, 0, 0, 0b00001111, 0, 0],
        [0, 0b10000000, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const TWENTY_PAST: Phrase = [
        [0, 0, 0b11111100, 0, 0, 0b00001111, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const TWENTY_FIVE_PAST: Phrase = [
        [0, 0, 0b11111101, 0, 0, 0b00001111, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const HALF_PAST: Phrase = [
        [0b00000011, 0, 0, 0, 0, 0b00001111, 0, 0],
        [0b11000000, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const TWENTY_FIVE_TO: Phrase = [
        [0, 0, 0b11111101, 0, 0, 0b01100000, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const TWENTY_TO: Phrase = [
        [0, 0, 0b11111100, 0, 0, 0b01100000, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const A_QUARTER_TO: Phrase = [
        [0, 0b10111111, 0, 0, 0, 0b01100000, 0, 0],
        [0, 0b10000000, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const TEN_TO: Phrase = [
        [0, 0, 0, 0, 0, 0b01100000, 0, 0],
        [0b00011100, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    pub const FIVE_TO: Phrase = [
        [0, 0, 0b00000001, 0, 0, 0b01100000, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];

    pub const IT_IS: Phrase = [[0b1101_1000, 0, 0, 0, 0, 0, 0, 0], EMPTY, EMPTY, EMPTY];
    pub const O_CLOCK: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0, 0, 0b0011_1111, 0, 0, 0]];
    pub const IN_THE_MORNING: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0b11011101, 0, 0],
        [0, 0, 0, 0, 0, 0b11111100, 0, 0],
    ];
    pub const IN_THE_AFTERNOON: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0b11011100, 0b11111111, 0],
        [0, 0, 0, 0, 0, 0, 0b10000000, 0],
    ];
    pub const IN_THE_EVENING: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0b11011100, 0, 0],
        [0, 0, 0, 0, 0, 0, 0b01111111, 0],
    ];
    pub const AT_NIGHT: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0, 0, 0b00110000],
        [0, 0, 0, 0, 0, 0, 0, 0b11111000],
    ];
}

impl TimeMode for DefaultMode {
    fn get_time_arrays(&self, hour: u32, minutes: u32) -> Phrase {
        let hour_phrase: Phrase = match (hour, minutes) {
            (0 | 12, 31..=60) | (1 | 13, 0..=30) => DefaultMode::ONE,
            (1 | 13, 31..=60) | (2 | 14, 0..=30) => DefaultMode::TWO,
            (2 | 14, 31..=60) | (3 | 15, 0..=30) => DefaultMode::THREE,
            (3 | 15, 31..=60) | (4 | 16, 0..=30) => DefaultMode::FOUR,
            (4 | 16, 31..=60) | (5 | 17, 0..=30) => DefaultMode::FIVE,
            (5 | 17, 31..=60) | (6 | 18, 0..=30) => DefaultMode::SIX,
            (6 | 18, 31..=60) | (7 | 19, 0..=30) => DefaultMode::SEVEN,
            (7 | 19, 31..=60) | (8 | 20, 0..=30) => DefaultMode::EIGHT,
            (8 | 20, 31..=60) | (9 | 21, 0..=30) => DefaultMode::NINE,
            (9 | 21, 31..=60) | (10 | 22, 0..=30) => DefaultMode::TEN,
            (10 | 22, 31..=60) | (11 | 23, 0..=30) => DefaultMode::ELEVEN,
            (11 | 23, 31..=60) | (0 | 12, 0..=30) => DefaultMode::TWELVE,
            _ => ALL,
        };

        let minute_phrase: Phrase = match minutes {
            0..=4 => DefaultMode::O_CLOCK,
            5..=9 => DefaultMode::FIVE_PAST,
            10..=14 => DefaultMode::TEN_PAST,
            15..=19 => DefaultMode::A_QUARTER_PAST,
            20..=24 => DefaultMode::TWENTY_PAST,
            25..=29 => DefaultMode::TWENTY_FIVE_PAST,
            30..=34 => DefaultMode::HALF_PAST,
            35..=39 => DefaultMode::TWENTY_FIVE_TO,
            40..=44 => DefaultMode::TWENTY_TO,
            45..=49 => DefaultMode::A_QUARTER_TO,
            50..=54 => DefaultMode::TEN_TO,
            55..=59 => DefaultMode::FIVE_TO,
            _ => ALL,
        };

        let modifier_phrase: Phrase = match hour {
            7..=11 => DefaultMode::IN_THE_MORNING,
            12..=15 => DefaultMode::IN_THE_AFTERNOON,
            16..=20 => DefaultMode::IN_THE_EVENING,
            21..=23 | 0..=6 => DefaultMode::AT_NIGHT,
            _ => ALL,
        };

        combine_phrases(&[
            DefaultMode::IT_IS,
            hour_phrase,
            minute_phrase,
            modifier_phrase
        ])
    }
}