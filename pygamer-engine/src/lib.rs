#![no_std]

use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use kuboble_core::{
    level_run::{render::LevelRunRenderer, Action, Direction, LevelRun, PieceSlid},
    level_select::{LevelProgress, LevelStatus},
    Level, Piece, Space, Vector,
};
use level_run::LevelRenderer;

mod level_run;

const FONT: MonoFont = embedded_graphics::mono_font::ascii::FONT_5X8;

trait VectorExt {
    fn into_point(self) -> Point;
}
impl VectorExt for Vector<u8> {
    fn into_point(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
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

pub trait PyGamer: Controller + LevelRunRenderer {}

struct Engine<P> {
    py_gamer: P,
}
impl<P: PyGamer> Engine<P> {
    pub fn run(&mut self) {
        let level_progress = LevelProgress::default();
        let level_info = level_progress.level_info(59);

        loop {
            // Setup the level run and perform the initial render
            let mut level_run = LevelRun::new(&level_info);
            level_run.render(&mut self.py_gamer);

            loop {
                match self.py_gamer.wait_for_action() {
                    Some(control_action) => {
                        let action = match control_action {
                            ControlAction::Move(d) => Action::Move(d),
                            ControlAction::A => Action::ChangeActivePiece,
                            ControlAction::B => Action::UndoMove,
                            ControlAction::Start => Action::Restart,
                            ControlAction::Select => Action::Restart,
                        };

                        let change = level_run.execute_action(action);
                        change.render(&mut self.py_gamer);

                        if change.winning_status.is_some() {
                            // Wait for user to proceed
                            match self.py_gamer.wait_for_proceed() {
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
}
