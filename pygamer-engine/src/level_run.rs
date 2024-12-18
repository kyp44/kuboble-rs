use crate::{assets, TryIntoSize, SPACE_RECT, SPACE_SIZE};
use crate::{
    display::FONT, ControlAction, Controller, GameOutput, GameResult, IntoPoint, PieceExt,
};
use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_framebuf::FrameBuf;
use embedded_sprites::sprite::Sprite;
use kuboble_core::BufferedRenderer;
use kuboble_core::{
    level_run::{render::LevelRunRenderer, Action, LevelRun, PieceSlid},
    level_select::{LevelInfo, LevelStatus},
    levels::MAX_STRIP_SIZE,
    Level, Piece, Space, Vector,
};

pub struct LevelRenderer<'a, G> {
    output: &'a mut G,
    level_rect: Rectangle,
    display_center: Point,
    at_max_moves: bool,
}
impl<'a, G: GameOutput> LevelRenderer<'a, G>
where
    G::Error: core::fmt::Debug,
{
    pub fn new(output: &'a mut G, level: &'a Level) -> Self {
        output.clear(Rgb565::BLACK).unwrap();

        let display_center = Rectangle::new(Point::zero(), output.size()).center();
        let level_size = level.size.try_into_size().unwrap() * SPACE_SIZE as u32;

        Self {
            output,
            level_rect: Rectangle::new(display_center - level_size / 2, level_size),
            display_center,
            at_max_moves: true,
        }
    }

    fn absolute_position(&self, level_position: Vector<u8>) -> Point {
        self.level_rect.top_left + level_position.into_point() * SPACE_SIZE as i32
    }

    fn draw_space_absolute<D: DrawTarget<Color = Rgb565>>(
        target: &mut D,
        point: Point,
        space: Space,
    ) where
        D::Error: core::fmt::Debug,
    {
        match space {
            Space::Void => return,
            Space::Wall => {
                Sprite::new(point, &assets::spaces::WALL)
                    .draw(target)
                    .unwrap();
            }
            Space::Free => {
                Sprite::new(point, &assets::spaces::FREE)
                    .draw(target)
                    .unwrap();
            }
            Space::Goal(piece) => {
                Sprite::new(point, &assets::spaces::FREE)
                    .draw(target)
                    .unwrap();

                Rectangle::new(point + Point::new(2, 2), SPACE_RECT.size - Size::new(3, 3))
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .stroke_color(piece.display_color())
                            .stroke_width(1)
                            .stroke_alignment(StrokeAlignment::Inside)
                            .build(),
                    )
                    .draw(target)
                    .unwrap();
            }
        }
    }

    fn draw_piece_absolute(&mut self, point: Point, piece: Piece, is_active: bool) {
        Sprite::new(point, &piece.image(is_active))
            .draw(self.output)
            .unwrap();
    }
}
impl<G: GameOutput> BufferedRenderer for LevelRenderer<'_, G> {
    fn flush(&mut self) {
        self.output.render();
    }
}
impl<G: GameOutput> LevelRunRenderer for LevelRenderer<'_, G>
where
    G::Error: core::fmt::Debug,
{
    fn draw_space(&mut self, position: Vector<u8>, space: Space) {
        Self::draw_space_absolute(self.output, self.absolute_position(position), space);
    }

    fn draw_piece(&mut self, position: Vector<u8>, piece: Piece, is_active: bool) {
        self.draw_piece_absolute(self.absolute_position(position), piece, is_active);
    }

    fn slide_piece(&mut self, piece_slid: &PieceSlid, is_active: bool) {
        // TODO: If we ever get to upgrade `embedded-graphics`, it now has a built-in framebuffer so maybe switch to that if possible.
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

            self.output.render();

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
    }

    fn update_active_piece(&mut self, piece: Piece) {
        self.output.indicate_active_piece(piece);
    }

    fn notify_win(&mut self, level_status: LevelStatus) {
        let mut fs: ArrayString<24> = ArrayString::new();
        write!(
            fs,
            "You win with {}/5 stars!",
            level_status.rating().num_stars()
        )
        .unwrap();

        self.output.indicate_win_rating(level_status.rating());

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
    }
}

pub fn play_level<C: Controller, G: GameOutput>(
    controller: &mut C,
    output: &mut G,
    level_info: &LevelInfo,
) -> GameResult<Option<LevelStatus>>
where
    G::Error: core::fmt::Debug,
{
    let mut level_run = LevelRun::new(level_info);
    let mut renderer = LevelRenderer::new(output, level_info.level);

    level_run.render(&mut renderer);

    loop {
        let action = match controller.wait_for_action()? {
            ControlAction::Move(dir) => Action::Move(dir),
            ControlAction::A => Action::ChangeActivePiece,
            ControlAction::B => Action::UndoMove,
            ControlAction::Start => Action::Restart,
            ControlAction::Select => return GameResult::Continue(None),
        };

        let change = level_run.execute_action(action);
        change.render(&mut renderer);

        if change.winning_status.is_some() {
            controller.wait_for_proceed()?;

            break GameResult::Continue(change.winning_status);
        }
    }
}
