#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::entry;

#[entry]
fn main() -> ! {
    let mut clock = lib::init();

    loop {
        clock.update_time();
    }
}
