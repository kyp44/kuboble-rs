#![no_std]

use embedded_graphics::{mono_font::MonoFont, pixelcolor::Rgb565, prelude::*};
use kuboble_core::{
    level_run::{Action, Direction, LevelRun},
    level_select::{LevelProgress, LevelSelector},
    LevelRating, Piece, Vector,
};
use level_run::LevelRenderer;

mod level_run;
mod level_select;

const FONT: MonoFont = embedded_graphics::mono_font::ascii::FONT_5X8;

trait VectorExt {
    fn into_point(self) -> Point;
}
impl<T: Into<i32>> VectorExt for Vector<T> {
    fn into_point(self) -> Point {
        Point::new(self.x.into(), self.y.into())
    }
}

trait PieceExt {
    fn display_color(&self) -> Rgb565;
}
impl PieceExt for Piece {
    fn display_color(&self) -> Rgb565 {
        match self {
            Piece::Green => Rgb565::GREEN,
            Piece::Orange => Rgb565::CSS_ORANGE,
            Piece::Blue => Rgb565::BLUE,
        }
    }
}

pub enum ControlAction {
    Move(Direction),
    A,
    B,
    Start,
    Select,
}

pub trait Controller {
    fn wait_for_action(&mut self) -> Option<ControlAction>;
    fn wait_for_proceed(&mut self) -> Option<()> {
        loop {
            match self.wait_for_action() {
                Some(ControlAction::A | ControlAction::Start) => break Some(()),
                None => break None,
                _ => {}
            }
        }
    }
}

pub trait GameDisplay: DrawTarget<Color = Rgb565> + OriginDimensions {
    fn flush(&mut self);
}

pub trait GameIndicator {
    fn indicate_active_piece(&mut self, piece: Piece);
    fn indicate_win_rating(&mut self, rating: LevelRating);
    fn indicate_nothing(&mut self);
}

pub trait GameOutput: GameDisplay + GameIndicator {
    // Slide speed in terms of pixel step size
    const SLIDE_SPEED: i32;

    // TODO: Just temporary
    fn print_test(&mut self, text: &str)
    where
        Self: Sized,
        <Self as DrawTarget>::Error: core::fmt::Debug,
    {
        embedded_graphics::text::Text::with_text_style(
            text,
            Point::zero(),
            embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                .font(&FONT)
                .text_color(Rgb565::WHITE)
                .background_color(Rgb565::BLACK)
                .build(),
            embedded_graphics::text::TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Left)
                .baseline(embedded_graphics::text::Baseline::Top)
                .build(),
        )
        .draw(self)
        .unwrap();
        self.flush();
    }
}

pub fn run_game<C: Controller, O: GameOutput>(mut controller: C, mut output: O)
where
    <O as DrawTarget>::Error: core::fmt::Debug,
{
    let level_progress = LevelProgress::default();
    let mut level_selector: Lev = LevelSelector::new(&mut level_progress);

    loop {
        // Setup the level run and perform the initial render
        let mut level_run = LevelRun::new(&level_info);
        let mut level_renderer = LevelRenderer::new(&mut output, level_info.level);
        level_run.render(&mut level_renderer);

        loop {
            match controller.wait_for_action() {
                Some(control_action) => {
                    let action = match control_action {
                        ControlAction::Move(d) => Action::Move(d),
                        ControlAction::A => Action::ChangeActivePiece,
                        ControlAction::B => Action::UndoMove,
                        ControlAction::Start => Action::Restart,
                        ControlAction::Select => Action::Restart,
                    };

                    let change = level_run.execute_action(action);
                    change.render(&mut level_renderer);

                    if change.winning_status.is_some() {
                        // Wait for user to proceed
                        match controller.wait_for_proceed() {
                            Some(_) => break,
                            None => return,
                        }
                    }
                }
                None => return,
            }
        }
    }
}
