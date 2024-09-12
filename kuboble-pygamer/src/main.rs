#![no_std]
#![no_main]
#![feature(let_chains)]

use controls::ControlAction;
use kuboble_core::level_run::{Action, LevelRun};
use kuboble_core::level_select::LevelProgress;
use pac::{CorePeripherals, Peripherals};
use pygamer::adc::Adc;
use pygamer::clock::GenericClockController;
use pygamer::delay::Delay;
use pygamer::pac::gclk::pchctrl::GEN_A;
use pygamer::timer::SpinTimer;
use pygamer::{entry, pac, Pins};
use renderer::LevelRenderer;

mod controls;
mod renderer;

#[entry]
fn main() -> ! {
    // Get the peripherals and pins
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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Initialize the display
    let (mut display, _backlight) = pins
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

    // Set up the neo-pixels driver
    // Note: This is the non-deprecated way but is jittery as commented in the example
    // here: https://github.com/atsamd-rs/atsamd/blob/master/boards/pygamer/examples/neopixel_rainbow_spi.rs
    // Maybe look back into this later so we don't have to use the deprecated SpinTimer.
    /* let tc4_clock = clocks.tc4_tc5(&clocks.gclk0()).unwrap();
    let mut neopixels_timer = TimerCounter::tc4_(&tc4_clock, peripherals.TC4, &mut peripherals.MCLK);
    neopixels_timer.start(3.mhz()); */
    let neopixels_timer = SpinTimer::new(5);
    let mut neopixels = pins.neopixel.init(neopixels_timer, &mut pins.port);

    // Setup level progress tracker
    let level_progress = LevelProgress::default();

    // Setup the controller
    let adc = Adc::adc1(
        peripherals.ADC1,
        &mut peripherals.MCLK,
        &mut clocks,
        GEN_A::GCLK11,
    );
    let mut controller = controls::Controller::new(
        adc,
        pins.joystick.init(&mut pins.port),
        pins.buttons.init(&mut pins.port),
    );

    let level_info = level_progress.level_info(59);
    loop {
        // Setup the renderer and render the initial level run
        let mut level_renderer = LevelRenderer::new(&mut display, &mut neopixels, level_info.level);

        // Setup the level run and perform the initial render
        let mut level_run = LevelRun::new(&level_info);
        level_run.render(&mut level_renderer);

        // Let the user play the level
        loop {
            let action = match controller.wait_for_action(&mut delay) {
                ControlAction::Move(d) => Action::Move(d),
                ControlAction::A => Action::ChangeActivePiece,
                ControlAction::B => Action::UndoMove,
                ControlAction::Start => Action::Restart,
                ControlAction::Select => break,
            };

            let change = level_run.execute_action(action);
            change.render(&mut level_renderer);

            if change.winning_status.is_some() {
                // Wait for user to proceed
                controller.wait_for_proceed(&mut delay);

                break;
            }
        }
    }
}
