#![no_std]
#![no_main]
#![feature(let_chains)]
#![feature(iter_advance_by)]

use controls::PyGamerController;
use core::cell::RefCell;
use kuboble_core::level_select::LevelProgress;
use output::PyGamerOutput;
use pac::{CorePeripherals, Peripherals};
use pygamer::hal::adc::Adc;
use pygamer::hal::clock::GenericClockController;
use pygamer::hal::delay::Delay;
use pygamer::pac::gclk::pchctrl::Genselect;
use pygamer::{entry, pac, Pins};
use pygamer_engine::run_game;

mod controls;
mod display;
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

    // Setup the SPI for the neopixels
    // TODO: Probably use DMA for interrupt robustness pending: https://github.com/smart-leds-rs/ws2812-spi-rs/pull/39
    let neopixels = pins.neopixel.init_spi(
        &mut clocks,
        // Unfortunately, the SPI driver requires a clock pin, even though it's not used by the
        // neopixels.
        pins.i2c.scl,
        peripherals.sercom2,
        &mut peripherals.mclk,
    );

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
