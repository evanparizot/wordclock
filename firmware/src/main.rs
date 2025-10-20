#![no_main]
#![no_std]

// This needs to exist otherwise an error will be thrown
use panic_semihosting as _;
extern crate alloc;

mod clock;
mod config;
mod modes;
mod setup;
mod times;

use rtic_monotonics::systick::prelude::*;

systick_monotonic!(Mono, 1000);

use crate::times::TimeMode;
use alloc::boxed::Box;
use modes::{make_by_name, REGISTRY};

#[cfg(not(any(feature = "mode-default", feature = "mode-am")))]
compile_error!("Enable at least one `mode-*` feature.");

const BUILD_MODE: Option<&'static str> = option_env!("WORDCLOCK_MODE");

fn make_mode() -> Box<dyn TimeMode + Send> {
    if let Some(sel) = BUILD_MODE {
        if let Some(m) = make_by_name(sel) {
            return m;
        }
    }

    (REGISTRY[0].make)()
}

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [EXTI1])]
mod app {
    use crate::{
        clock::Clock,
        config::{DAY_INTENSITY, NIGHT_END_HOUR, NIGHT_INTENSITY, NIGHT_START_HOUR},
        Mono,
    };
    // use alloc::boxed::Box;
    // use cortex_m::peripheral::SYST;
    use rtic_monotonics::fugit::ExtU32;
    use rtic_monotonics::fugit::MillisDurationU32;
    use cortex_m_semihosting::hprintln;
    use hal::{
        gpio::{Input, PA0, PA2, PB2, PB3},
        // time::duration::Milliseconds,
    };
    use rtic_monotonics::Monotonic;

    const DEBOUNCE_MS: u32 = 200;
    const RENDER_PERIOD_MS: u32 = 500;

    const HOLD_MS: u32 = 700;
    const REPEAT_MS: u32 = 120;

    #[shared]
    struct Shared {
        clock: Clock,
        last_hour_press: Option<<Mono as rtic_monotonics::Monotonic>::Instant>,
        last_minute_press: Option<<Mono as rtic_monotonics::Monotonic>::Instant>,
        last_rendered_minute: Option<u32>,
        blink: bool,
        last_brightness: Option<u8>,

        last_hour_hold_start: Option<<Mono as rtic_monotonics::Monotonic>::Instant>,
        last_minute_hold_start: Option<<Mono as rtic_monotonics::Monotonic>::Instant>,
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

        let mode = crate::make_mode();

        let (clock, hour_button, minute_button) = crate::setup::init(ctx.core, ctx.device, mode);

        let _ = render_tick::spawn();

        (
            Shared {
                clock,
                last_hour_press: None,
                last_minute_press: None,
                last_rendered_minute: None,
                blink: false,
                last_brightness: None,
                last_hour_hold_start: None,
                last_minute_hold_start: None,
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

    #[task(priority = 1, shared = [clock, last_rendered_minute, blink, last_brightness])]
    async fn render_tick(mut ctx: render_tick::Context) {
        loop {
            // -------------------------------------------------------------
            // Brightness Checks

            let applied = ctx.shared.clock.lock(|c| {
                c.apply_time_based_brightness(
                    NIGHT_START_HOUR,
                    NIGHT_END_HOUR,
                    DAY_INTENSITY,
                    NIGHT_INTENSITY,
                )
            });

            let mut changed = false;
            ctx.shared.last_brightness.lock(|lb| {
                if lb.map(|old| old != applied).unwrap_or(true) {
                    *lb = Some(applied);
                    changed = true;
                }
            });

            // -------------------------------------------------------------
            // Render Checks

            // (1) Check current minute
            let minute_now = ctx.shared.clock.lock(|c| {
                let (_h, m, _s) = c.get_time();
                m as u32
            });

            let remainder = (minute_now % 5) as u8;
            let blink_on = ctx.shared.blink.lock(|b| { *b = !*b; *b });

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
                ctx.shared.clock.lock(|c| c.set_heartbeat(blink_on));
                // let on = ctx.shared.blink.lock(|b| {
                //     *b = !*b;
                //     *b
                // });
                // ctx.shared.clock.lock(|c| c.set_heartbeat(on));
            }

            let mut mask: u8 = 0;
            if remainder > 0 {
                for i in 0..(remainder - 1) {
                    mask |= 1 << i;
                }

                if blink_on {
                    mask |= 1 << (remainder - 1);
                }
            }

            set_minute_leds_mask(mask);

            // (Last) Re-arm next tick
            Mono::delay(MillisDurationU32::millis(RENDER_PERIOD_MS)).await;
        }
    }

    // https://docs.rs/stm32f3xx-hal/latest/stm32f3xx_hal/enum.interrupt.html
    #[task(binds = EXTI0, local = [hour_button], shared = [clock, last_hour_press, last_hour_hold_start])]
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

            if hour_button_pressed() {
                let mut spawn = false;
                ctx.shared.last_hour_hold_start.lock(|s| {
                    if s.is_none() {
                        *s = Some(Mono::now());
                        spawn = true;
                    }
                });
                if spawn {
                    let _ = hour_hold_repeat::spawn();
                }
            }
        }
    }

    #[task(binds = EXTI3, local = [minute_button], shared = [clock, last_minute_press, last_minute_hold_start])]
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

            if minute_button_pressed() {
                let mut spawn = false;
                ctx.shared.last_minute_hold_start.lock(|s| {
                    if s.is_none() {
                        *s = Some(Mono::now());
                        spawn = true;
                    }
                });
                if spawn {
                    let _ = minute_hold_repeat::spawn();
                }
            }
        }
    }

    #[task(priority = 1, shared = [clock, last_minute_hold_start])]
    async fn minute_hold_repeat(mut ctx: minute_hold_repeat::Context) {
        loop {
            if !minute_button_pressed() {
                break; // released
            }
            let ready = ctx.shared.last_minute_hold_start.lock(|start| {
                start
                    .map(|t0| (Mono::now() - t0).to_millis() >= HOLD_MS)
                    .unwrap_or(false)
            });
            if ready {
                ctx.shared.clock.lock(|c| {
                    c.add_minutes(1);
                    c.update_display_time();
                });
                Mono::delay(REPEAT_MS.millis()).await;
            } else {
                Mono::delay(20u32.millis()).await; // poll until HOLD_MS reached
            }
        }
        // clear hold marker on release
        ctx.shared.last_minute_hold_start.lock(|s| *s = None);
    }

    #[task(priority = 1, shared = [clock, last_hour_hold_start])]
    async fn hour_hold_repeat(mut ctx: hour_hold_repeat::Context) {
        loop {
            if !hour_button_pressed() {
                break; // released
            }
            let ready = ctx.shared.last_hour_hold_start.lock(|start| {
                start
                    .map(|t0| (Mono::now() - t0).to_millis() >= HOLD_MS)
                    .unwrap_or(false)
            });
            if ready {
                ctx.shared.clock.lock(|c| {
                    c.add_hours(1);
                    c.update_display_time();
                });
                Mono::delay(REPEAT_MS.millis()).await;
            } else {
                Mono::delay(20u32.millis()).await;
            }
        }
        ctx.shared.last_hour_hold_start.lock(|s| *s = None);
    }

    #[inline]
    fn minute_button_pressed() -> bool {
        unsafe { (*hal::pac::GPIOB::ptr()).idr.read().idr3().bit_is_clear() }
    }

    #[inline]
    fn hour_button_pressed() -> bool {
        unsafe { (*hal::pac::GPIOA::ptr()).idr.read().idr0().bit_is_clear() }
    }

    const MIN_LED_PORT: *const hal::pac::gpioa::RegisterBlock = hal::pac::GPIOA::ptr();
    const MIN_LED_PINS: [u8; 4] = [1,2,3,4];

    #[inline]
    fn set_led_pa(pin: u8, on: bool) {
        unsafe {
            let gpioa = &*MIN_LED_PORT;
            if on {
                gpioa.bsrr.write(|w| w.bits(1u32 << pin));
            } else {
                gpioa.bsrr.write(|w| w.bits(1u32 << (pin + 16)));
            }
        }
    }

    #[inline]
    fn set_minute_leds_mask(mask: u8) {
        for (i, &pin) in MIN_LED_PINS.iter().enumerate() {
            let on = (mask >> i) & 1 == 1;
            set_led_pa(pin, on);
        }
    }
}
