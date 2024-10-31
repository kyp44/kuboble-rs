#![no_std]
#![no_main]
#![feature(let_chains)]

use atsamd_hal::dmac::{DmaController, PriorityLevel};
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

    // Setup a DMA channel
    let channels = DmaController::init(peripherals.dmac, &mut peripherals.pm).split();

    // Setup the neopixels
    let neopixels = {
        // Configure the SERCOM2 clock
        let gclk0 = clocks.gclk0();
        let sercom2_clock = clocks.sercom2_core(&gclk0).unwrap();

        // Setup the PADS
        let pads = spi::Pads::default()
            .sclk(pins.i2c.scl)
            .data_out(pins.neopixel.neopixel);

        // Configure the SPI
        let config = spi::Config::new(
            &mut peripherals.mclk,
            peripherals.sercom2,
            pads,
            sercom2_clock.freq(),
            //3.MHz(),
        )
        .spi_mode(spi::MODE_0)
        .baud(3.MHz());

        // Configure the DMA channel
        let channel = channels.0.init(PriorityLevel::Lvl3);

        let neopixels_spi = config
            .enable()
            .with_tx_channel(channel)
            .into_panic_on_read();

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
