
use crate::times::{EMPTY, Phrase, ALL, TimeMode, combine_phrases};

use super::default::DefaultMode;

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
