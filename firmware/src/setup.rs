extern crate alloc;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

use alloc::boxed::Box;
use ds323x::Ds323x;
use hal::{delay::Delay, i2c::I2c, prelude::*, spi::Spi, gpio::{Gpioa, Input, U, Pin, Edge}};
use max7219::MAX7219;
use crate::{clock::Clock, times::TimeMode};


pub fn init(
    cp: cortex_m::Peripherals,
    dp: hal::pac::Peripherals,
    mode: Box<dyn TimeMode + Send>,
) -> (
    Clock, 
    Pin<Gpioa, U<0>, Input>,
    Pin<Gpioa, U<2>, Input>,
) {

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut syscfg = dp.SYSCFG.constrain(&mut rcc.apb2);
    let mut exti = dp.EXTI;

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let clocks = rcc
        .cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .pclk2(32.MHz())
        .freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);


    //  /$$$$$$$  /$$   /$$ /$$$$$$$$ /$$$$$$$$ /$$$$$$  /$$   /$$  /$$$$$$ 
    // | $$__  $$| $$  | $$|__  $$__/|__  $$__//$$__  $$| $$$ | $$ /$$__  $$
    // | $$  \ $$| $$  | $$   | $$      | $$  | $$  \ $$| $$$$| $$| $$  \__/
    // | $$$$$$$ | $$  | $$   | $$      | $$  | $$  | $$| $$ $$ $$|  $$$$$$ 
    // | $$__  $$| $$  | $$   | $$      | $$  | $$  | $$| $$  $$$$ \____  $$
    // | $$  \ $$| $$  | $$   | $$      | $$  | $$  | $$| $$\  $$$ /$$  \ $$
    // | $$$$$$$/|  $$$$$$/   | $$      | $$  |  $$$$$$/| $$ \  $$|  $$$$$$/
    // |_______/  \______/    |__/      |__/   \______/ |__/  \__/ \______/ 

    // https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/gpio_interrupts.rs

    let mut hour_button = gpioa.pa0.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
    syscfg.select_exti_interrupt_source(&hour_button);
    hour_button.trigger_on_edge(&mut exti, Edge::Rising);
    hour_button.enable_interrupt(&mut exti);

    let mut minute_button = gpioa.pa2.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
    syscfg.select_exti_interrupt_source(&minute_button);
    minute_button.trigger_on_edge(&mut exti, Edge::Rising);
    minute_button.enable_interrupt(&mut exti);


    //  /$$$$$$  /$$$$$$   /$$$$$$
    // |_  $$_/ /$$__  $$ /$$__  $$
    //   | $$  |__/  \ $$| $$  \__/
    //   | $$    /$$$$$$/| $$
    //   | $$   /$$____/ | $$
    //   | $$  | $$      | $$    $$
    //  /$$$$$$| $$$$$$$$|  $$$$$$/
    // |______/|________/ \______/
    //
    // DS3231
    // Default time is 01/01/00T00:00:00
    // https://github.com/eldruin/driver-examples/blob/master/stm32f3-discovery/examples/ds3231-f3.rs

    let mut scl =
        gpiob
            .pb6
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 100_000.Hz(), clocks, &mut rcc.apb1);
    let clock = Ds323x::new_ds3231(i2c);


    //   /$$$$$$  /$$$$$$$  /$$$$$$
    //  /$$__  $$| $$__  $$|_  $$_/
    // | $$  \__/| $$  \ $$  | $$
    // |  $$$$$$ | $$$$$$$/  | $$
    //  \____  $$| $$____/   | $$
    //  /$$  \ $$| $$        | $$
    // |  $$$$$$/| $$       /$$$$$$
    //  \______/ |__/      |______/
    //
    // https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/spi.rs
    // https://github.com/almindor/max7219-examples/blob/master/examples/display_spi.rs
    // https://github.com/almindor/max7219

    let cs = gpiob
        .pb12
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let sck = gpiob
        .pb13
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
    let miso = gpiob
        .pb14
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
    let mosi = gpiob
        .pb15
        .into_af_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);

    let spi = Spi::new(dp.SPI2, (sck, miso, mosi), 3.MHz(), clocks, &mut rcc.apb1);

    let displays = 4;
    let mut display = MAX7219::from_spi_cs(displays, spi, cs).unwrap();

    display.power_on().unwrap();
    for a in 0..displays {
        display.clear_display(a).unwrap();
        display.set_intensity(a, 5).unwrap();
    }

    (Clock {
        display,
        clock,
        delay,
        mode,
    }, hour_button, minute_button)
}
