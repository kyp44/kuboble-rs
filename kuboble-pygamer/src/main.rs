#![no_std]
#![no_main]

use pygamer::timer::{SpinTimer, TimerCounter};
//use panic_halt as _;
use pygamer::{entry, hal, pac, Pins};

use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay};
use pac::{CorePeripherals, Peripherals};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{SmartLedsWrite, RGB, RGB8};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let mut pins = Pins::new(peripherals.PORT).split();

    // neopixels
    let timer = SpinTimer::new(4);
    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);

    let mut colors = [
        RGB::new(255, 0, 0),
        RGB::new(0, 255, 0),
        RGB::new(0, 0, 255),
    ]
    .into_iter()
    .cycle();

    //let x = kuboble_core::tester();

    loop {
        neopixel.write(colors.clone().take(5)).unwrap();

        delay.delay_ms(1000u32);
        colors.next();
    }
}
