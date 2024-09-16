use core::fmt::Write;
use derive_new::new;
use embedded_graphics::{
    pixelcolor::{PixelColor, Rgb565},
    prelude::*,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay,
    SimulatorEvent, Window,
};
use kuboble_core::{
    level_run::{render::LevelRunRenderer, Direction, LevelRun, PieceSlid},
    level_select::{LevelProgress, LevelStatus},
    Level, Piece, Space, Vector,
};
use pygamer_engine::{run_game, ControlAction, Controller, GameDisplay, GameIndicator, GameOutput};
use std::{cell::RefCell, convert::Infallible};

#[derive(new)]
struct SimulatorController<'a> {
    window: &'a RefCell<Window>,
}
impl Controller for SimulatorController<'_> {
    fn wait_for_action(&mut self) -> Option<ControlAction> {
        let mut window = self.window.borrow_mut();

        loop {
            for event in window.events() {
                return Some(match event {
                    SimulatorEvent::KeyDown {
                        keycode,
                        keymod: _,
                        repeat: _,
                    } => match keycode {
                        Keycode::Up => ControlAction::Move(Direction::Up),
                        Keycode::Down => ControlAction::Move(Direction::Down),
                        Keycode::Left => ControlAction::Move(Direction::Left),
                        Keycode::Right => ControlAction::Move(Direction::Right),
                        Keycode::A => ControlAction::A,
                        Keycode::S => ControlAction::B,
                        Keycode::Z => ControlAction::Start,
                        Keycode::X => ControlAction::Select,
                        _ => continue,
                    },
                    SimulatorEvent::Quit => return None,
                    _ => continue,
                });
            }
        }
    }
}

struct SimulatorOutput<'a> {
    display: SimulatorDisplay<Rgb565>,
    window: &'a RefCell<Window>,
}
impl<'a> SimulatorOutput<'a> {
    pub fn new(window: &'a RefCell<Window>) -> Self {
        Self {
            display: SimulatorDisplay::<Rgb565>::new(Size::new(160, 128)),
            window,
        }
    }
}
impl OriginDimensions for SimulatorOutput<'_> {
    fn size(&self) -> Size {
        self.display.size()
    }
}
impl DrawTarget for SimulatorOutput<'_> {
    type Color = Rgb565;

    type Error = <SimulatorDisplay<Rgb565> as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels)
    }
}
impl GameDisplay for SimulatorOutput<'_> {
    fn flush(&mut self) {
        self.window.borrow_mut().update(&self.display);
    }
}
impl GameIndicator for SimulatorOutput<'_> {
    fn indicate_active_piece(&mut self, _piece: Piece) {
        // Do nothing because there are no indicators
    }

    fn indicate_win_rating(&mut self, _rating: kuboble_core::LevelRating) {
        // Do nothing because there are no indicators
    }

    fn indicate_nothing(&mut self) {
        // Do nothing because there are no indicators
    }
}
impl GameOutput for SimulatorOutput<'_> {
    const SLIDE_SPEED: i32 = 10;
}

fn main() -> anyhow::Result<()> {
    let window = RefCell::new(Window::new("Kuboble", &OutputSettings::default()));

    run_game(
        SimulatorController::new(&window),
        SimulatorOutput::new(&window),
    );

    Ok(())
}
