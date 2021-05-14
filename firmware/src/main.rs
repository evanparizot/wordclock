#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lib::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = lib::init();

    let mut half_period = 500_u16;
    let v_half_period = Volatile::new(&mut half_period);

    loop {
        leds[0].on().ok();
        delay.delay_ms(v_half_period.read());

        leds[0].off().ok();
        delay.delay_ms(v_half_period.read());
    }
    //
    // let _y;
    // let x = 42;
    // _y = x;
    //
    // // infinite loop; just so we don't leave this stack frame
    // loop {}
}
