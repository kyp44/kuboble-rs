#![no_std]
#![no_main]

use controls::ControlAction;
use heapless::String;
use kuboble_core::{Action, Board, LEVELS};
use pac::{CorePeripherals, Peripherals};
use pygamer::adc::Adc;
use pygamer::clock::GenericClockController;
use pygamer::delay::Delay;
use pygamer::pac::gclk::pchctrl::GEN_A;
use pygamer::prelude::_embedded_hal_blocking_delay_DelayMs;
use pygamer::prelude::*;
use pygamer::timer::TimerCounter;
use pygamer::{entry, hal, pac, Pins};
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

    // Setup board
    let mut board = Board::from(&LEVELS[0]);

    // Set up the neo-pixels
    // TODO: Some of this stuff needs moved to render, just test code.
    use pygamer::prelude::*;
    use smart_leds::{SmartLedsWrite, RGB};

    let gclk0 = &clocks.gclk0();
    let x = clocks.tc4_tc5(gclk0).unwrap();
    let mut z = TimerCounter::tc4_(&x, peripherals.TC4, &mut peripherals.MCLK);
    z.start(3.mhz());
    //let z = hal::timer::SpinTimer::new(4);
    let mut neopixels = pins.neopixel.init(z, &mut pins.port);

    /*
    // Setup the board renderer and render the initial board
    let mut level_renderer = LevelRenderer::new(&mut display, &mut neopixels, board.level());
    board.render(&mut level_renderer);

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

    loop {
        let action = match controller.wait_for_action(&mut delay) {
            ControlAction::Move(d) => Action::Move(d),
            ControlAction::A => Action::ChangeActivePiece,
            ControlAction::B => Action::UndoMove,
            ControlAction::Start => Action::Restart,
            ControlAction::Select => todo!(),
        };

        board.execute_action(action).render(&mut level_renderer);
    }*/

    let mut colors = [
        RGB::new(255, 0, 0),
        RGB::new(0, 255, 0),
        RGB::new(0, 0, 255),
    ]
    .into_iter()
    .cycle();

    loop {
        neopixels.write(colors.clone().take(5)).unwrap();

        delay.delay_ms(3000u32);
        colors.next();
    }
}
