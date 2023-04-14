// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;
extern crate alloc;

mod clock;
mod config;
mod setup;
mod times;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    // use hal::prelude::*;

    use crate::{clock::Clock, config::am::AdrianMorgan};

    #[shared]
    struct Shared {
        clock: Clock,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp = ctx.device;
        let cp = ctx.core;

        let clock = crate::setup::init(cp, dp, alloc::boxed::Box::new(AdrianMorgan {}));

        (Shared { clock }, Local {}, init::Monotonics())
    }

    #[idle(shared = [clock])]
    fn idle(ctx: idle::Context) -> ! {
        let mut clock = ctx.shared.clock;
        hprintln!("Starting up!");
        loop {
            clock.lock(|clock| {
                clock.update_display_time();
            });
        }
    }
}

// #[entry]
// fn main() -> ! {
//     let mut clock = init::init(Box::new(AdrianMorgan {}));

//     loop {
//         clock.update_display_time();
//     }
// }
