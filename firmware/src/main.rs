#![no_main]
#![no_std]

// This needs to exist otherwise an error will be thrown
use panic_semihosting as _;
extern crate alloc;

mod clock;
mod config;
mod setup;
mod times;

use rtic_monotonics::systick::prelude::*;

systick_monotonic!(Mono, 1000);

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [EXTI1])]
mod app {
    use crate::{clock::Clock, config::am::AdrianMorgan, Mono};
    // use alloc::boxed::Box;
    // use cortex_m::peripheral::SYST;
    use cortex_m_semihosting::hprintln;
    use hal::{
        gpio::{Input, PA0, PA2, PB2, PB3},
        // time::duration::Milliseconds,
    };
    use rtic_monotonics::Monotonic;

    const DEBOUNCE_MS: u32 = 200;
    const RENDER_PERIOD_MS: u32 = 500;

    #[shared]
    struct Shared {
        clock: Clock,
        last_hour_press: Option<<Mono as rtic_monotonics::Monotonic>::Instant>,
        last_minute_press: Option<<Mono as rtic_monotonics::Monotonic>::Instant>,
        last_rendered_minute: Option<u32>,
        blink: bool,
    }

    #[local]
    struct Local {
        hour_button: PA0<Input>,
        minute_button: PB3<Input>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        hprintln!("Initializing!");

        crate::setup::init_allocator();
        let mode = alloc::boxed::Box::new(AdrianMorgan {});


        let (clock, hour_button, minute_button) =
            crate::setup::init(ctx.core, ctx.device, mode);

        let _ = render_tick::spawn();

        (
            Shared {
                clock,
                last_hour_press: None,
                last_minute_press: None,
                last_rendered_minute: None,
                blink: false,
            },
            Local {
                hour_button,
                minute_button,
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("Time Idle!");

        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(priority = 1, shared = [clock, last_rendered_minute, blink])]
    async fn render_tick(mut ctx: render_tick::Context) {
        loop {
            // (1) Check current minute
            let minute_now = ctx.shared.clock.lock(|c| {
                let (_h, m, _s) = c.get_time();
                m as u32
            });

            // (2) Decide if we need to render
            let mut need_render = false;
            ctx.shared.last_rendered_minute.lock(|last| {
                if last.map(|lm| lm != minute_now).unwrap_or(true) {
                    *last = Some(minute_now);
                    need_render = true;
                }
            });

            // (3) Render (only if necessary)
            if need_render {
                ctx.shared.clock.lock(|c| {
                    c.update_display_time();
                });
            } else {
                ctx.shared.blink.lock(|b| {
                    *b = !*b;
                });
            }

            // (4) Re-arm next tick
            use rtic_monotonics::fugit::MillisDurationU32;
            Mono::delay(MillisDurationU32::millis(RENDER_PERIOD_MS)).await;
        }
    }

    // https://docs.rs/stm32f3xx-hal/latest/stm32f3xx_hal/enum.interrupt.html
    #[task(binds = EXTI0, local = [hour_button], shared = [clock, last_hour_press])]
    fn update_hour(mut ctx: update_hour::Context) {
        ctx.local.hour_button.clear_interrupt();

        let now = Mono::now();
        let mut should_act = false;

        ctx.shared.last_hour_press.lock(|last| match *last {
            Some(prev) => {
                let elapsed = (now - prev).to_millis();
                if elapsed >= DEBOUNCE_MS {
                    *last = Some(now);
                    should_act = true;
                }
            }
            None => {
                *last = Some(now);
                should_act = true;
            }
        });

        if should_act {
            ctx.shared.clock.lock(|c| {
                c.add_hours(1);
                c.update_display_time();
            });
        }
    }

    #[task(binds = EXTI3, local = [minute_button], shared = [clock, last_minute_press])]
    fn update_minutes(mut ctx: update_minutes::Context) {
        ctx.local.minute_button.clear_interrupt();
        let now = Mono::now();
        let mut should_act = false;

        ctx.shared.last_minute_press.lock(|last| match *last {
            Some(prev) => {
                let elapsed = (now - prev).to_millis();
                if elapsed >= DEBOUNCE_MS {
                    *last = Some(now);
                    should_act = true;
                }
            }
            None => {
                *last = Some(now);
                should_act = true;
            }
        });

        if should_act {
            ctx.shared.clock.lock(|c| {
                c.add_minutes(1);
                c.update_display_time();
            });
        }
    }
}
