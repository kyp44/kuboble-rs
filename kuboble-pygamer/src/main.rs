#![no_std]
#![no_main]
#![feature(let_chains)]

use controls::PyGamerController;
use core::cell::RefCell;
use kuboble_core::level_select::LevelProgress;
use output::PyGamerOutput;
use pac::{CorePeripherals, Peripherals};
use pygamer::adc::Adc;
use pygamer::clock::GenericClockController;
use pygamer::delay::Delay;
use pygamer::pac::gclk::pchctrl::GEN_A;
use pygamer::timer::SpinTimer;
use pygamer::{entry, pac, Pins};
use pygamer_engine::run_game;

mod controls;
mod output;

#[entry]
fn main() -> ! {
    // Get the peripherals and pins and setup clocks
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = Pins::new(peripherals.PORT).split();
    // TODO: use sleeping delay here for battery life? Evidently worth it even for delays of like 50ms
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Initialize the display
    let (display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    // Need to share the delay
    let delay = RefCell::new(delay);

    // Set up the neo-pixels driver
    // Note: This is the non-deprecated way but is jittery as commented in the example
    // here: https://github.com/atsamd-rs/atsamd/blob/master/boards/pygamer/examples/neopixel_rainbow_spi.rs
    // Maybe look back into this later so we don't have to use the deprecated SpinTimer.
    /* let tc4_clock = clocks.tc4_tc5(&clocks.gclk0()).unwrap();
    let mut neopixels_timer = TimerCounter::tc4_(&tc4_clock, peripherals.TC4, &mut peripherals.MCLK);
    neopixels_timer.start(3.mhz()); */
    let neopixels_timer = SpinTimer::new(4);
    let neopixels = pins.neopixel.init(neopixels_timer, &mut pins.port);

    // TODO Need to read and later write this from EEPROM
    let mut level_progress = LevelProgress::default();

    run_game(
        PyGamerController::new(
            &delay,
            Adc::adc1(
                peripherals.ADC1,
                &mut peripherals.MCLK,
                &mut clocks,
                GEN_A::GCLK11,
            ),
            pins.joystick.init(&mut pins.port),
            pins.buttons.init(&mut pins.port),
        ),
        PyGamerOutput::new(display, neopixels),
        &mut level_progress,
    );

    loop {}
}
