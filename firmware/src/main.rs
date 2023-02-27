#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::{Clock, entry};

#[entry]
fn main() -> ! {
    let mut clock: Clock = lib::init();

    loop {
        
    }

}
