pub fn combine_phrases<const N: usize, const M: usize>(phrases: &[[[u8; N]; M]]) -> [[u8; N]; M] {
    let mut result = [[0; N]; M];

    match phrases.len() {
        0 => return result,
        1 => return *phrases.get(0).unwrap(),
        _ => (),
    }

    for s in 0..phrases.len() {
        let phrase = phrases[s];

        for m in 0..M {
            let square = phrase[m];
            let new_square = or_arrays::<N>(result[m], square);
            result[m] = new_square;
        }
    }

    result
}

fn or_arrays<const N: usize>(one: [u8; N], two: [u8; N]) -> [u8; N] {
    let mut result = [0; N];
    for n in 0..N {
        result[n] = one[n] | two[n];
    }

    result
}

pub type Phrase = [[u8; 8]; 4];

const EMPTY: [u8; 8] = [u8::MIN; 8];
const FULL: [u8; 8] = [u8::MAX; 8];

pub const BLANK: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
pub const ALL: Phrase = [FULL, FULL, FULL, FULL];

pub enum Mode {
    FiveMinute,
    OneMinute,
}

pub struct FiveMinuteMode {}

impl FiveMinuteMode {
    pub const ONE: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TWO: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const THREE: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const FOUR: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const FIVE: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const SIX: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const SEVEN: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const EIGHT: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const NINE: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TEN: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const ELEVEN: Phrase = [EMPTY, [0b1111_0000, 0, 0, 0, 0, 0, 0, 0], EMPTY, EMPTY];
    pub const TWELVE: Phrase = [[0b1111_0000, 0, 0, 0, 0, 0, 0, 0], EMPTY, EMPTY, EMPTY];

    pub const FIVE_PAST: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TEN_PAST: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const QUARTER_PAST: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TWENTY_PAST: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TWENTY_FIVE_PAST: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TWENTY_FIVE_TO: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const HALF_PAST: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TWENTY_TO: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const A_QUARTER_TO: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const TEN_TO: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const FIVE_TO: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];

    pub const IT_IS: Phrase = [[0, 0, 0, 0b0111_0000, 0, 0, 0, 0], EMPTY, EMPTY, EMPTY];
    pub const O_CLOCK: Phrase = [EMPTY, EMPTY, EMPTY, [0, 0, 0, 0, 0b0011_1111, 0, 0, 0]];
    pub const IN_THE_MORNING: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const IN_THE_AFTERNOON: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const IN_THE_EVENING: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
    pub const AT_NIGHT: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
}
