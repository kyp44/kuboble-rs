#![no_std]
#![no_main]
#![feature(let_chains)]

use controls::PyGamerController;
use core::cell::RefCell;
use kuboble_core::level_select::LevelProgress;
use output::PyGamerOutput;
use pac::{CorePeripherals, Peripherals};
use pygamer::hal::adc::Adc;
use pygamer::hal::clock::GenericClockController;
use pygamer::hal::delay::Delay;
use pygamer::hal::prelude::*;
use pygamer::hal::sercom::spi;
use pygamer::pac::gclk::pchctrl::Genselect;
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
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port).split();
    // TODO: use sleeping delay here for battery life? Evidently worth it even for delays of like 50ms
    //let x = SleepingDelay::new();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Initialize the display
    let (display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.sercom4,
            &mut peripherals.mclk,
            peripherals.tc2,
            &mut delay,
        )
        .unwrap();

    // Setup the neopixels
    let neopixels = {
        let pads: output::Test = spi::Pads::default()
            .sclk(pins.i2c.scl)
            .data_out(pins.neopixel.neopixel);

        let config = spi::Config::new(&mut peripherals.mclk, peripherals.sercom2, pads, 3.MHz())
            .spi_mode(spi::MODE_0);

        // TODO: Not reaching this line
        panic!();

        let neopixels_spi = config.enable().into_panic_on_read();

        ws2812_spi::Ws2812::new(neopixels_spi)
    };

    // Need to share the delay
    let delay = RefCell::new(delay);

    // TODO Need to read and later write this from EEPROM
    let mut level_progress = LevelProgress::default();

    run_game(
        PyGamerController::new(
            &delay,
            Adc::adc1(
                peripherals.adc1,
                &mut peripherals.mclk,
                &mut clocks,
                Genselect::Gclk11,
            ),
            pins.joystick.init(),
            pins.buttons.init(),
        ),
        PyGamerOutput::new(display, neopixels),
        &mut level_progress,
    );

    loop {}
}
