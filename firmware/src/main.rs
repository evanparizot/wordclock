#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::{entry};
use state::{StateMachine, State};
mod state;


#[entry]
fn main() -> ! {
    let (mut clock) = lib::init();


    // Read switch position to know which time mode to use

    // Display iniatialization sequence?
    loop {
        // Step 1 - Check inputs
        clock.update_time();

        // Step 2 - Check set time

    }
}
