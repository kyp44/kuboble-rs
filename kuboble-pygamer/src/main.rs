#![no_std]
#![no_main]
#![feature(let_chains)]

use controls::PyGamerController;
use core::cell::RefCell;
use kuboble_core::level_select::LevelProgress;
use pac::{CorePeripherals, Peripherals};
use pygamer::hal::adc::Adc;
use pygamer::hal::clock::GenericClockController;
use pygamer::hal::delay::Delay;
use pygamer::hal::prelude::*;
use pygamer::hal::sercom::spi;
use pygamer::hal::timer::TimerCounter;
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

    // Need to share the delay
    let delay = RefCell::new(delay);

    let pads: output::Test<_, _, _> = spi::Pads::default()
        .sclk(pins.sd_cs_pin.into())
        .data_out(pins.neopixel.into());

    /* let config = spi::Config::new(
        &mut peripherals.mclk,
        peripherals.sercom2,
        pads,
        clocks.sercom2_core(&clocks.gclk0()).unwrap().freq(),
    )
    .spi_mode(spi::MODE_0)
    .baud(3.MHz());

    let neopixels_spi = config.enable().into_panic_on_read();

    let mut neopixels: () = ws2812_spi::Ws2812::new(neopixels_spi);

    use smart_leds::{SmartLedsWrite, RGB};
    neopixels.write();

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
    ); */

    loop {}
}
