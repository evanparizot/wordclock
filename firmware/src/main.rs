#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::{entry, iprintln};

#[entry]
fn main() -> ! {
    let (mut clock, mut itm) = lib::init();

    // Set time

    // iprintln!(&mut itm.stim[0], "Hello there");



    loop {
        // Step 1 - Check inputs

        // Step 2 - Check set time

    }
}
