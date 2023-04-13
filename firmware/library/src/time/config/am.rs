use crate::time::times::{combine_phrases, Phrase, TimeMode, ALL, EMPTY};

/*
ITxISxHA LFxTENxx
AxQUARTE Rxxxxxxx
TWENTYxF IVExxxxx
HAPPYxxx BIRTHDAY
xxxxxANN IVERSARY
xTOxPAST MORGANxx
xxxxxxAN DxADRIAN
THREExxx TWOxxONE

xxSEVENx xxEIGHTx
FOURxxTW ELVEFIVE
xxSIXxxx NINExxxx
ELEVENxx xxxTENxx
xxxxxxxx xxOCLOCK
INxTHExM ORNINGxx
AFTERNOO NEVENING
xxATxMID NIGHTxxx
*/

pub struct AdrianMorgan {}

impl AdrianMorgan {
    const ONE: Phrase = [EMPTY, [0, 0, 0, 0, 0, 0, 0, 0b00000111], EMPTY, EMPTY];
    const TWO: Phrase = [EMPTY, [0, 0, 0, 0, 0, 0, 0, 0b11100000], EMPTY, EMPTY];
    const THREE: Phrase = [[0, 0, 0, 0, 0, 0, 0, 0b11111000], EMPTY, EMPTY, EMPTY];
    const FOUR: Phrase = [EMPTY, EMPTY, [0, 0b11110000, 0, 0, 0, 0, 0, 0], EMPTY];
    const FIVE: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0b00001111, 0, 0, 0, 0, 0, 0]];
    const SIX: Phrase = [EMPTY, EMPTY, [0, 0, 0b00111000, 0, 0, 0, 0, 0], EMPTY];
    const SEVEN: Phrase = [EMPTY, EMPTY, [0b00111110, 0, 0, 0, 0, 0, 0, 0], EMPTY];
    const EIGHT: Phrase = [EMPTY, EMPTY, EMPTY, [0b00111110, 0, 0, 0, 0, 0, 0, 0]];
    const NINE: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0b11110000, 0, 0, 0, 0, 0]];
    const TEN: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0, 0b00011100, 0, 0, 0, 0]];
    const ELEVEN: Phrase = [EMPTY, EMPTY, [0, 0, 0, 0b11111100, 0, 0, 0, 0], EMPTY];
    const TWELVE: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0b00000011, 0, 0, 0, 0, 0, 0],
        [0, 0b11110000, 0, 0, 0, 0, 0, 0],
    ];

    const FIVE_PAST: Phrase = [
        [0, 0, 0b00000001, 0, 0, 0b00001111, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const TEN_PAST: Phrase = [
        [0, 0, 0, 0, 0, 0b00001111, 0, 0],
        [0b00011100, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const A_QUARTER_PAST: Phrase = [
        [0, 0b10111111, 0, 0, 0, 0b00001111, 0, 0],
        [0, 0b10000000, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const TWENTY_PAST: Phrase = [
        [0, 0, 0b11111100, 0, 0, 0b00001111, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const TWENTY_FIVE_PAST: Phrase = [
        [0, 0, 0b11111101, 0, 0, 0b00001111, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const HALF_PAST: Phrase = [
        [0b00000011, 0, 0, 0, 0, 0b00001111, 0, 0],
        [0b11000000, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const TWENTY_FIVE_TO: Phrase = [
        [0, 0, 0b11111101, 0, 0, 0b01100000, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const TWENTY_TO: Phrase = [
        [0, 0, 0b11111100, 0, 0, 0b01100000, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const A_QUARTER_TO: Phrase = [
        [0, 0b10111111, 0, 0, 0, 0b01100000, 0, 0],
        [0, 0b10000000, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const TEN_TO: Phrase = [
        [0, 0, 0, 0, 0, 0b01100000, 0, 0],
        [0b00011100, 0, 0, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const FIVE_TO: Phrase = [
        [0, 0, 0b00000001, 0, 0, 0b01100000, 0, 0],
        [0, 0, 0b11100000, 0, 0, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];

    const IT_IS: Phrase = [[0b1101_1000, 0, 0, 0, 0, 0, 0, 0], EMPTY, EMPTY, EMPTY];
    const O_CLOCK: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0, 0, 0b0011_1111, 0, 0, 0]];
    const IN_THE_MORNING: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0b11011101, 0, 0],
        [0, 0, 0, 0, 0, 0b11111100, 0, 0],
    ];
    const IN_THE_AFTERNOON: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0b11011100, 0b11111111, 0],
        [0, 0, 0, 0, 0, 0, 0b10000000, 0],
    ];
    const IN_THE_EVENING: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0b11011100, 0, 0],
        [0, 0, 0, 0, 0, 0, 0b01111111, 0],
    ];
    const AT_NIGHT: Phrase = [
        EMPTY,
        EMPTY,
        [0, 0, 0, 0, 0, 0, 0, 0b00110000],
        [0, 0, 0, 0, 0, 0, 0, 0b11111000],
    ];

    const _HAPPY: Phrase = [[0, 0, 0, 0b11111000, 0, 0, 0, 0], EMPTY, EMPTY, EMPTY];
    const _BIRTHDAY: Phrase = [EMPTY, [0, 0, 0, 0b11111111, 0, 0, 0, 0], EMPTY, EMPTY];
    const _ANNIVERSARY: Phrase = [
        [0, 0, 0, 0, 0b00000111, 0, 0, 0],
        [0, 0, 0, 0, 0b11111111, 0, 0, 0],
        EMPTY,
        EMPTY,
    ];
    const _MORGAN: Phrase = [EMPTY, [0, 0, 0, 0, 0, 0b11111100, 0, 0], EMPTY, EMPTY];
    const _AND: Phrase = [
        [0, 0, 0, 0, 0, 0, 0b00000011, 0],
        [0, 0, 0, 0, 0, 0, 0b10000000, 0],
        EMPTY,
        EMPTY,
    ];
    const _ADRIAN: Phrase = [EMPTY, [0, 0, 0, 0, 0, 0, 0b00111111, 0], EMPTY, EMPTY];
}

impl TimeMode for AdrianMorgan {
    fn get_time_arrays(&self, hour: u32, minutes: u32) -> Phrase {
        let hour_phrase: Phrase = match (hour, minutes) {
            (0 | 12, 31..=60) | (1 | 13, 0..=30) => AdrianMorgan::ONE,
            (1 | 13, 31..=60) | (2 | 14, 0..=30) => AdrianMorgan::TWO,
            (2 | 14, 31..=60) | (3 | 15, 0..=30) => AdrianMorgan::THREE,
            (3 | 15, 31..=60) | (4 | 16, 0..=30) => AdrianMorgan::FOUR,
            (4 | 16, 31..=60) | (5 | 17, 0..=30) => AdrianMorgan::FIVE,
            (5 | 17, 31..=60) | (6 | 18, 0..=30) => AdrianMorgan::SIX,
            (6 | 18, 31..=60) | (7 | 19, 0..=30) => AdrianMorgan::SEVEN,
            (7 | 19, 31..=60) | (8 | 20, 0..=30) => AdrianMorgan::EIGHT,
            (8 | 20, 31..=60) | (9 | 21, 0..=30) => AdrianMorgan::NINE,
            (9 | 21, 31..=60) | (10 | 22, 0..=30) => AdrianMorgan::TEN,
            (10 | 22, 31..=60) | (11 | 23, 0..=30) => AdrianMorgan::ELEVEN,
            (11 | 23, 31..=60) | (0 | 12, 0..=30) => AdrianMorgan::TWELVE,
            _ => ALL,
        };

        let minute_phrase: Phrase = match minutes {
            0..=4 => AdrianMorgan::O_CLOCK,
            5..=9 => AdrianMorgan::FIVE_PAST,
            10..=14 => AdrianMorgan::TEN_PAST,
            15..=19 => AdrianMorgan::A_QUARTER_PAST,
            20..=24 => AdrianMorgan::TWENTY_PAST,
            25..=29 => AdrianMorgan::TWENTY_FIVE_PAST,
            30..=34 => AdrianMorgan::HALF_PAST,
            35..=39 => AdrianMorgan::TWENTY_FIVE_TO,
            40..=44 => AdrianMorgan::TWENTY_TO,
            45..=49 => AdrianMorgan::A_QUARTER_TO,
            50..=54 => AdrianMorgan::TEN_TO,
            55..=59 => AdrianMorgan::FIVE_TO,
            _ => ALL,
        };

        let modifier_phrase: Phrase = match hour {
            7..=11 => AdrianMorgan::IN_THE_MORNING,
            12..=15 => AdrianMorgan::IN_THE_AFTERNOON,
            16..=20 => AdrianMorgan::IN_THE_EVENING,
            21..=23 | 0..=6 => AdrianMorgan::AT_NIGHT,
            _ => ALL,
        };

        combine_phrases(&[
            AdrianMorgan::IT_IS,
            hour_phrase,
            minute_phrase,
            modifier_phrase
        ])
    }
}
