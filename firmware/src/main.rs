#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::{Clock, entry};

#[entry]
fn main() -> ! {
    let mut clock: Clock = lib::init();

    // Set time

    loop {

        // Step 1 - Check inputs

        // Step 2 - Check set time
    }

}
