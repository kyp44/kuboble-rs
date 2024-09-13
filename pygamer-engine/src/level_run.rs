use crate::VectorExt;
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
    level_run::{render::LevelRunRenderer, Direction, LevelRun, PieceSlid},
    level_select::{LevelProgress, LevelStatus},
    Level, Piece, Space, Vector,
};

const SPACE_SIZE: u32 = 12;
static SPACE_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(SPACE_SIZE, SPACE_SIZE));

pub struct LevelRenderer<'a, D> {
    display: &'a mut D,
    level_origin: Point,
    display_center: Point,
    at_max_moves: bool,
}
impl<'a, D: DrawTarget<Color = Rgb565> + OriginDimensions> LevelRenderer<'a, D>
where
    D::Error: core::fmt::Debug,
{
    pub fn new(display: &'a mut D, level: &'a Level) -> Self {
        display.clear(Rgb565::BLACK).unwrap();

        let display_center = Rectangle::new(Point::zero(), display.size()).center();

        Self {
            display,
            level_origin: display_center - level.size.into_point() * (SPACE_SIZE as i32 / 2),
            display_center,
            at_max_moves: true,
        }
    }

    fn absolute_position(&self, level_position: Vector<u8>) -> Point {
        self.level_origin + level_position.into_point() * SPACE_SIZE as i32
    }

    fn set_active_piece(&mut self, piece: Piece) {
        // TODO Need to traitify this probably
    }

    // TODO: Just temporary
    pub fn print_test(&mut self, text: &str) {
        Text::with_text_style(
            text,
            Point::zero(),
            MonoTextStyle::new(&FONT, Rgb565::BLUE),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(self.display)
        .unwrap();
    }
}
impl<'a, D: DrawTarget<Color = Rgb565> + OriginDimensions> LevelRunRenderer for LevelRenderer<'_, D>
where
    D::Error: core::fmt::Debug,
{
    fn draw_space(&mut self, position: Vector<u8>, space: Space) {
        const SPACE_COLOR: Rgb565 = Rgb565::CSS_GRAY;

        let style = match space {
            Space::Void => PrimitiveStyle::with_fill(Rgb565::BLACK),
            Space::Wall => PrimitiveStyle::with_fill(Rgb565::WHITE),
            Space::Free => PrimitiveStyle::with_fill(SPACE_COLOR),
            Space::Goal(piece) => PrimitiveStyleBuilder::new()
                .stroke_color(piece.display_color())
                .stroke_width(2)
                .fill_color(SPACE_COLOR)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        };

        SPACE_RECT
            .translate(self.absolute_position(position))
            .into_styled(style)
            .draw(self.display)
            .unwrap();
    }

    fn draw_piece(&mut self, position: Vector<u8>, piece: Piece, is_active: bool) {
        Circle::new(self.absolute_position(position), SPACE_SIZE)
            .into_styled(PrimitiveStyle::with_fill(piece.display_color()))
            .draw(self.display)
            .unwrap();

        if is_active {
            Circle::with_center(
                self.absolute_position(position) + SPACE_RECT.center(),
                SPACE_SIZE / 2,
            )
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(self.display)
            .unwrap();

            self.set_active_piece(piece);
        }
    }

    fn slide_piece(&mut self, piece_slid: &PieceSlid, is_active: bool) {
        // TODO: Test code, 4 horizontal tile strip in the middle of the level (level 11).
        /* let mut framebuf_backend = [Rgb565::BLACK; 4 * 12 * 12];
        let mut framebuf = FrameBuf::new(
            &mut framebuf_backend,
            4 * SPACE_SIZE as usize,
            SPACE_SIZE as usize,
        );
        Text::with_text_style(
            "Tickle tester!",
            Point::zero(),
            MonoTextStyleBuilder::new()
                .font(&FONT)
                .text_color(Rgb565::YELLOW)
                .background_color(Rgb565::BLUE)
                .build(),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(&mut framebuf)
        .unwrap();
        Circle::new(Point::new(SPACE_SIZE as i32 * 3, 0), SPACE_SIZE)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_HOT_PINK))
            .draw(&mut framebuf)
            .unwrap();
        self.display
            .fill_contiguous(
                &Rectangle::new(
                    self.level_position(Vector::new(1, 2)),
                    Size::new(4 * SPACE_SIZE, SPACE_SIZE),
                ),
                framebuf_backend,
            )
            .unwrap(); */

        // TODO: Animate this with constant slide time? Observe how the web version does it
        self.draw_space(piece_slid.starting_position, piece_slid.starting_space);
        self.draw_piece(
            piece_slid.starting_position
                + piece_slid.muv.direction.as_vector() * piece_slid.distance.try_into().unwrap(),
            piece_slid.muv.piece,
            is_active,
        );
    }

    fn update_num_moves(&mut self, num_moves: u8, at_maximum: bool) {
        let mut fs: ArrayString<12> = ArrayString::new();

        let num_chars = if self.at_max_moves == at_maximum {
            // Just update the number
            write!(fs, "{}  ", num_moves).unwrap();
            7
        } else {
            // Need to update the number and texts
            write!(fs, "Moves: {}  ", num_moves).unwrap();
            0
        };
        self.at_max_moves = at_maximum;

        Text::with_text_style(
            &fs,
            Point::new(
                FONT.character_size.width as i32 * num_chars,
                self.display.size().height as i32 - 1,
            ),
            MonoTextStyleBuilder::new()
                .font(&FONT)
                .text_color(if at_maximum {
                    Rgb565::RED
                } else {
                    Rgb565::WHITE
                })
                .background_color(Rgb565::BLACK)
                .build(),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Bottom)
                .build(),
        )
        .draw(self.display)
        .unwrap();
    }

    fn update_constants(&mut self, level_num: u16, goal: u8) {
        let mut fs: ArrayString<10> = ArrayString::new();

        // Draw level number
        write!(fs, "Level {}", level_num).unwrap();

        Text::with_text_style(
            &fs,
            Point::new(self.display_center.x, 0),
            MonoTextStyle::new(&FONT, Rgb565::WHITE),
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(self.display)
        .unwrap();

        // Draw the goal
        fs.clear();
        write!(fs, "Goal: {}", goal).unwrap();
        let size = self.display.size();

        Text::with_text_style(
            &fs,
            Point::new(size.width as i32, size.height as i32) - Point::new(1, 1),
            MonoTextStyle::new(&FONT, Rgb565::WHITE),
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Bottom)
                .build(),
        )
        .draw(self.display)
        .unwrap();
    }

    fn notify_win(&mut self, level_status: LevelStatus) {
        let mut fs: ArrayString<24> = ArrayString::new();
        write!(
            fs,
            "You win with {}/5 stars!",
            level_status.rating().num_stars()
        )
        .unwrap();

        // TODO: This needs finalized with location and stars.
        Text::with_text_style(
            &fs,
            Point::new(self.display_center.x, 10),
            MonoTextStyle::new(&FONT, Rgb565::YELLOW),
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(self.display)
        .unwrap();
    }
}
