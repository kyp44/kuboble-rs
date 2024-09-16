use crate::{GameOutput, PieceExt, VectorExt, FONT};
use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_framebuf::FrameBuf;
use kuboble_core::{
    level_run::{render::LevelRunRenderer, PieceSlid},
    level_select::LevelStatus,
    levels::MAX_STRIP_SIZE,
    Level, Piece, Space, Vector,
};

const SPACE_SIZE: u32 = 14;
static SPACE_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(SPACE_SIZE, SPACE_SIZE));

pub struct LevelRenderer<'a, G> {
    output: &'a mut G,
    level_origin: Point,
    display_center: Point,
    at_max_moves: bool,
}
impl<'a, G: GameOutput> LevelRenderer<'a, G>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    pub fn new(output: &'a mut G, level: &'a Level) -> Self {
        output.clear(Rgb565::BLACK).unwrap();

        let display_center = Rectangle::new(Point::zero(), output.size()).center();

        Self {
            output,
            level_origin: display_center - level.size.into_point() * (SPACE_SIZE as i32 / 2),
            display_center,
            at_max_moves: true,
        }
    }

    fn absolute_position(&self, level_position: Vector<u8>) -> Point {
        self.level_origin + level_position.into_point() * SPACE_SIZE as i32
    }

    fn draw_space_absolute<D: DrawTarget<Color = Rgb565>>(
        target: &mut D,
        point: Point,
        space: Space,
    ) where
        D::Error: core::fmt::Debug,
    {
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
            .translate(point)
            .into_styled(style)
            .draw(target)
            .unwrap();
    }

    fn draw_piece_absolute(&mut self, point: Point, piece: Piece, is_active: bool) {
        Circle::new(point, SPACE_SIZE)
            .into_styled(PrimitiveStyle::with_fill(piece.display_color()))
            .draw(self.output)
            .unwrap();

        if is_active {
            Circle::with_center(point + SPACE_RECT.center(), SPACE_SIZE / 2)
                .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
                .draw(self.output)
                .unwrap();

            self.output.indicate_active_piece(piece);
        }

        self.output.flush();
    }
}
impl<'a, G: GameOutput> LevelRunRenderer for LevelRenderer<'_, G>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    fn draw_space(&mut self, position: Vector<u8>, space: Space) {
        Self::draw_space_absolute(self.output, self.absolute_position(position), space);
        self.output.flush();
    }

    fn draw_piece(&mut self, position: Vector<u8>, piece: Piece, is_active: bool) {
        self.draw_piece_absolute(self.absolute_position(position), piece, is_active);
    }

    fn slide_piece(&mut self, piece_slid: &PieceSlid, is_active: bool) {
        // Create a frame buffer for the slide strip, though this will likely be larger than needed
        let mut framebuf_backend =
            [Rgb565::BLACK; MAX_STRIP_SIZE * SPACE_SIZE as usize * SPACE_SIZE as usize];

        let (width, height) = if piece_slid.muv.direction.is_horizontal() {
            (MAX_STRIP_SIZE * SPACE_SIZE as usize, SPACE_SIZE as usize)
        } else {
            (SPACE_SIZE as usize, MAX_STRIP_SIZE * SPACE_SIZE as usize)
        };
        let mut framebuf = FrameBuf::new(&mut framebuf_backend, width, height);

        let variable_point = |var, con| {
            if piece_slid.muv.direction.is_horizontal() {
                Point::new(var, con)
            } else {
                Point::new(con, var)
            }
        };

        // Draw the background spaces into the frame buffer
        for (i, space) in piece_slid.strip_spaces.iter().enumerate() {
            Self::draw_space_absolute(
                &mut framebuf,
                variable_point(i as i32 * SPACE_SIZE as i32, 0),
                *space,
            )
        }

        // Slide the piece
        let dir_vector = piece_slid.muv.direction.as_vector().into_point();
        let strip_abs_point = self.absolute_position(piece_slid.strip_top_left);

        let final_pos = piece_slid.slide_distance() as i32 * SPACE_SIZE as i32;
        let initial_buffer_point = if (-piece_slid.muv.direction).is_forward() {
            variable_point(final_pos, 0)
        } else {
            Point::new(0, 0)
        };

        let mut bg_pos = 0;
        loop {
            let bg_buffer_point = initial_buffer_point + dir_vector * bg_pos;
            let piece_pos = final_pos.min(bg_pos + G::SLIDE_SPEED);

            // Draw background at the previous space
            self.output
                .fill_contiguous(
                    &SPACE_RECT.translate(strip_abs_point + bg_buffer_point),
                    SPACE_RECT
                        .points()
                        .map(|sp| framebuf.get_color_at(bg_buffer_point + sp)),
                )
                .unwrap();

            // Draw piece at the next position
            self.draw_piece_absolute(
                strip_abs_point + initial_buffer_point + dir_vector * piece_pos,
                piece_slid.muv.piece,
                is_active,
            );

            self.output.flush();

            // Are we done?
            if piece_pos >= final_pos {
                break;
            }

            bg_pos += G::SLIDE_SPEED;
        }
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
                self.output.size().height as i32 - 1,
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
        .draw(self.output)
        .unwrap();

        self.output.flush();
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
        .draw(self.output)
        .unwrap();

        // Draw the goal
        fs.clear();
        write!(fs, "Goal: {}", goal).unwrap();
        let size = self.output.size();

        Text::with_text_style(
            &fs,
            Point::new(size.width as i32, size.height as i32) - Point::new(1, 1),
            MonoTextStyle::new(&FONT, Rgb565::WHITE),
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Bottom)
                .build(),
        )
        .draw(self.output)
        .unwrap();

        self.output.flush();
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
        .draw(self.output)
        .unwrap();

        self.output.flush();
    }
}
