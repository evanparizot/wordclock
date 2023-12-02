#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate alloc;

mod clock;
mod config;
mod setup;
mod times;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    use hal::gpio::{PA0, Input};
    use alloc::boxed::Box;

    use crate::{clock::Clock, config::am::AdrianMorgan};

    #[shared]
    struct Shared {
        clock: Clock,
    }

    #[local]
    struct Local {
        button: PA0<Input> 
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Initializing!");
        let (
            clock, 
            button
        ) = crate::setup::init(ctx.core, ctx.device, Box::new(AdrianMorgan {}));

        (Shared { clock }, Local { button }, init::Monotonics())
    }

    #[idle(shared = [clock])]
    fn idle(ctx: idle::Context) -> ! {
        let mut clock = ctx.shared.clock;
        hprintln!("Time Idle!");
        loop {
            clock.lock(|clock| {
                clock.update_display_time();
            });
        }
    }

    #[task(binds = EXTI0, local = [button], shared = [clock])]
    fn update_time(ctx: update_time::Context) {
        ctx.local.button.clear_interrupt();
    }
}
