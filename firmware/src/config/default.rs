use crate::times::{EMPTY, Phrase};


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