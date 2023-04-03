
fn combine(phrases: &[&[&[u8; 8]; 4]]) -> &[&[u8; 8]; 4] {
    if phrases.len() == 1 {
        return phrases.get(0).as_ref();
    }

    for phrase in phrases {
        
    }

}

const BLANK: &[u8; 8] = &[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
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
    BLANK,
    BLANK,
    BLANK
];
