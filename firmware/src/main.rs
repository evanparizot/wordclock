#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::{entry, iprintln};

#[entry]
fn main() -> ! {
    let (mut clock, mut itm) = lib::init();

    // Set time

    iprintln!(&mut itm.stim[0], "Hello there");


    clock.set_intensity(0, u8::MAX);
    clock.write_raw(
        0,
        &[
            0b0001_1000,
            0b0001_1000,
            0b0001_1000,
            0b1111_1111,
            0b1111_1111,
            0b0001_1000,
            0b0001_1000,
            0b0001_1000,
        ],
    );

    clock.clear(0);
    clock.clear(1);
    clock.clear(2);

    loop {
        // Step 1 - Check inputs

        // Step 2 - Check set time

    }
}
