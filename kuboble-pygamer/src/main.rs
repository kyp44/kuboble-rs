#![no_std]
#![no_main]

use heapless::String;
use kuboble_core::{Board, LEVELS};
use pac::{CorePeripherals, Peripherals};
use pygamer::adc::Adc;
use pygamer::clock::GenericClockController;
use pygamer::delay::Delay;
use pygamer::pac::gclk::pchctrl::GEN_A;
use pygamer::prelude::_embedded_hal_blocking_delay_DelayMs;
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

    // Setup the board renderer and render the initial board
    let mut renderer = LevelRenderer::new(&mut display, board.level());
    board.render(&mut renderer);

    // Setup the controller
    let adc = Adc::adc1(
        peripherals.ADC1,
        &mut peripherals.MCLK,
        &mut clocks,
        GEN_A::GCLK11,
    );
    let mut controller = controls::Controller::new(adc, pins.joystick.init(&mut pins.port));

    loop {
        use core::fmt::Write;

        let (x, y) = controller.wait_for_action(&mut delay);
        let mut fs: String<50> = String::new();
        write!(fs, "{}, {}    ", x, y).unwrap();
        renderer.print_test(&fs);
    }
    /*

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

    loop {
        neopixel.write(colors.clone().take(5)).unwrap();

        delay.delay_ms(3000u32);
        colors.next();
    } */
}
