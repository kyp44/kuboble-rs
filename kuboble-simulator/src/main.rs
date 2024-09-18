use derive_new::new;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};
use kuboble_core::{level_run::Direction, level_select::LevelProgress, Piece};
use pygamer_engine::{
    run_game, ControlAction, Controller, GameDisplay, GameIndicator, GameOutput, GameResult,
};
use std::{cell::RefCell, fs::File, u32};

#[derive(new)]
struct SimulatorController<'a> {
    window: &'a RefCell<Window>,
}
impl Controller for SimulatorController<'_> {
    fn wait_for_action(&mut self) -> GameResult<ControlAction> {
        let mut window = self.window.borrow_mut();

        loop {
            for event in window.events() {
                return GameResult::Continue(match event {
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
                    SimulatorEvent::Quit => return GameResult::Exit,
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

const PROGRESS_FILE_NAME: &str = "level-progress.json";

fn load_progress() -> Result<LevelProgress, anyhow::Error> {
    Ok(serde_json::from_reader(File::open(PROGRESS_FILE_NAME)?)?)
}

fn main() -> anyhow::Result<()> {
    let mut level_progress = load_progress().unwrap_or_else(|_| LevelProgress::default());

    let window = RefCell::new(Window::new(
        "Kuboble",
        &OutputSettings {
            scale: 3,
            pixel_spacing: 0,
            theme: BinaryColorTheme::Default,
            max_fps: u32::MAX,
        },
    ));

    run_game(
        SimulatorController::new(&window),
        SimulatorOutput::new(&window),
        &mut level_progress,
    );

    // Save out the level progress
    // TODO: Need to do this as part of the engine somehow
    serde_json::to_writer(File::create(PROGRESS_FILE_NAME)?, &level_progress)?;

    Ok(())
}
