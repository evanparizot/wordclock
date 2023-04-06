fn get_time<const N: usize, const M: usize>(phrases: &[[[u8; N]; M]]) -> [[u8; N]; M] {

    let mut result = [[0; N]; M];

    match phrases.len() {
        0 => return result,
        1 => return *phrases.get(0).unwrap(),
        _ => ()
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

// fn get_time<const N: usize, const M: usize, const U: usize>(times: &[[[u8; N]; M]; U]) -> [[u8; N]; M] {

//     let mut result = [[0; N]; M];

    // let mut result = [[0; N]; M];

    

    // let blah: [[u8; N]; M] = *times.into_iter().reduce(|&a, &b| {
    //     let mut result: [[u8; N]; M] = [[0; N]; M];
    //     for m in 0..M {
    //         result[m] = or_arrays::<N>(a[m], b[m]);
    //     }

    //     &result
    // }).unwrap();

    // blah



//     let blah: [[u8; N]; M] = *times.into_iter().reduce(|&a, &b| {
//         let mut result: [[u8; N]; M] = [[0; N]; M];
//         for m in 0..M {
//             result[m] = or_arrays::<N>(a[m], b[m]);
//         }

//         &result
//     }).unwrap();

//     blah
// }


const EMPTY: &[u8; 8] = &[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
];

const EMPTY_TWO: [u8; 8] = [
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
];

const NONE_TWO: [[u8; 8]; 4] = [
    EMPTY_TWO, EMPTY_TWO, EMPTY_TWO, EMPTY_TWO
];

const NONE: &[&[u8; 8]; 4] = &[
    EMPTY, EMPTY, EMPTY, EMPTY
];

const IT_IS: &[&[u8; 8]; 4] = &[
    &[
        0b1101_1000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ],
    EMPTY,
    EMPTY,
    EMPTY,
];

const A_QUARTER: &[&[u8; 8]; 4] = &[
    EMPTY,
    &[
        0b0111_1111,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ],
    EMPTY,
    EMPTY
];