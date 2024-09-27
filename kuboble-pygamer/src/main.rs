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
use pygamer::sleeping_delay::SleepingDelay;
use pygamer::timer::SpinTimer;
use pygamer::{entry, pac, Pins};
use pygamer_engine::run_game;
use rtic_monotonics::systick::prelude::*;

mod controls;
mod output;

systick_monotonic!(Mono, 1000);

#[rtic::app(device = pygamer::pac, dispatchers = [TC0])]
mod app {
    use crate::{output, Mono};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Get the peripherals and pins and setup clocks
        let mut clocks = pygamer::clock::GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = Pins::new(peripherals.PORT).split();
        // TODO: use sleeping delay here for battery life? Evidently worth it even for delays of like 50ms
        //let x = SleepingDelay::new();
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

        neopixels_test::spawn(0).unwrap();
        display_test::spawn().unwrap();

        Mono::start(cx.core.SYST, 12_000_000);

        (Shared {}, Local {})
    }

    #[task(priority = 1)]
    async fn neopixels_test(
        _: neopixels_test::Context,
        mut neopixels: crate::output::NeoPixels<pygamer::timer::SpinTimer>,
    ) {
        output::neopixels_test(neopixels).await
    }

    #[task(priority = 1)]
    async fn display_test(_: display_test::Context, mut display: output::DisplayDriver) {
        output::display_test(display).await
    }

    // TODO: Do a test about idle if there are suspended higher priority software tasks
    #[idle]
    fn idle(cx: idle::Context) -> ! {
        // error: no `local_to_foo` field in `idle::LocalResources`
        // _cx.local.local_to_foo += 1;

        // error: no `local_to_bar` field in `idle::LocalResources`
        // _cx.local.local_to_bar += 1;

        loop {
            cortex_m::asm::nop();
        }
    }
}

/* #[entry]
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
    //let x = SleepingDelay::new();
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

    let mut executor = Executor::new();

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
 */
