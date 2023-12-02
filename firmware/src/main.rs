#![deny(unsafe_code)]
#![no_main]
#![no_std]

// This needs to exist otherwise an error will be thrown
use panic_semihosting as _;
extern crate alloc;

mod clock;
mod config;
mod setup;
mod times;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    use hal::gpio::{PA0, PA2, Input, PB2, PB3};
    use alloc::boxed::Box;

    use crate::{clock::Clock, config::am::AdrianMorgan};

    #[shared]
    struct Shared {
        clock: Clock,
    }

    #[local]
    struct Local {
        hour_button: PA0<Input>,
        minute_button: PB3<Input>
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Initializing!");
        let (
            clock, 
            hour_button,
            minute_button
        ) = crate::setup::init(ctx.core, ctx.device, Box::new(AdrianMorgan {}));

        (Shared { clock }, Local { hour_button, minute_button}, init::Monotonics())
    }

    #[idle(shared = [clock])]
    fn idle(ctx: idle::Context) -> ! {
        let mut clock = ctx.shared.clock;
        hprintln!("Time Idle!");
        loop {
            clock.lock(|clock| {
                clock.update_display_time();
                let (h, m, s) = clock.get_time();
                hprintln!("{:?}, {:?}, {:?}", h, m, s);
            });
        }
    }

    // https://docs.rs/stm32f3xx-hal/latest/stm32f3xx_hal/enum.interrupt.html
    #[task(binds = EXTI0, local = [hour_button], shared = [clock])]
    fn update_hour(ctx: update_hour::Context) {
        ctx.local.hour_button.clear_interrupt();

        let mut clock = ctx.shared.clock;
        clock.lock(|clock| {
            hprintln!("Add hours(s)");
            clock.add_hours(1);
        });
    }

    #[task(binds = EXTI3, local = [minute_button], shared = [clock])]
    fn update_minutes(ctx: update_minutes::Context) {
        ctx.local.minute_button.clear_interrupt();

        let mut clock = ctx.shared.clock;
        clock.lock(|clock| {
            hprintln!("Add minutes(s)");
            clock.add_minutes(1);
        });
    }
}
