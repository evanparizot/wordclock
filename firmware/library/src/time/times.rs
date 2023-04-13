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

pub const EMPTY: [u8; 8] = [u8::MIN; 8];
pub const FULL: [u8; 8] = [u8::MAX; 8];
pub const BLANK: Phrase = [EMPTY, EMPTY, EMPTY, EMPTY];
pub const ALL: Phrase = [FULL, FULL, FULL, FULL];

pub trait TimeMode {
    fn get_time_arrays(&self, hours: u32, minutes: u32) -> Phrase;
}
