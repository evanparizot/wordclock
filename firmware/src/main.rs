#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use lib::{entry, time::config::am::AdrianMorgan};

#[entry]
fn main() -> ! {
    let mut clock = lib::init(Box::new(AdrianMorgan {}));

    loop {
        clock.update_time();
    }
}
