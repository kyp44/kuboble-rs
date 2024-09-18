use crate::{
    assets::{self, stars::STAR_SIZE},
    ControlAction, Controller, GameOutput, GameResult, LevelRatingExt, PieceExt, FONT, SPACE_RECT,
    SPACE_SIZE,
};
use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_sprites::sprite::Sprite;
use kuboble_core::{
    level_run::Direction as ControlDirection,
    level_select::{
        render::LevelSelectRenderer, Action, Direction, Filter, LevelInfo, LevelSelector,
        LevelSlotInfo,
    },
    LevelRating, Piece,
};
use strum::IntoEnumIterator;

const SLOT_HEIGHT: u32 = 14;
const SLOT_WIDTH: u32 = 160;
const WINDOW_SIZE: usize = 5;
static SLOT_RECT: Rectangle = Rectangle::new(
    Point::new(0, SLOT_HEIGHT as i32),
    Size::new(SLOT_WIDTH, SLOT_HEIGHT),
);

pub struct SelectRenderer<'a, G> {
    output: &'a mut G,
}
impl<'a, G: GameOutput> SelectRenderer<'a, G>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    pub fn new(output: &'a mut G) -> Self {
        output.clear(Rgb565::BLACK).unwrap();

        Self { output }
    }

    fn slot_rectangle(&self, position: u8) -> Rectangle {
        SLOT_RECT.translate(Point::new(0, position as i32 * SLOT_HEIGHT as i32))
    }

    fn fill_slot(&mut self, position: u8, stroke: Option<Rgb565>, background: Rgb565) {
        let mut style = PrimitiveStyleBuilder::new().fill_color(background).build();

        if let Some(color) = stroke {
            style.stroke_color = Some(color);
            style.stroke_width = 1;
        }

        self.slot_rectangle(position)
            .into_styled(style)
            .draw(self.output)
            .unwrap();
    }
}
impl<'a, G: GameOutput> LevelSelectRenderer for SelectRenderer<'_, G>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo) {
        const MARGIN: i32 = 3;

        let mut fs: ArrayString<8> = ArrayString::new();

        match level_slot_info {
            LevelSlotInfo::Empty(p) => self.fill_slot(*p, None, Rgb565::BLACK),
            LevelSlotInfo::Level {
                level_info,
                position,
                is_active,
            } => {
                self.fill_slot(
                    *position,
                    Some(if *is_active {
                        Rgb565::WHITE
                    } else {
                        Rgb565::CSS_SLATE_GRAY
                    }),
                    if *is_active {
                        Rgb565::CSS_DARK_GREEN
                    } else {
                        Rgb565::CSS_DARK_SLATE_GRAY
                    },
                );

                let slot_rectangle = self.slot_rectangle(*position);

                // Draw level number
                write!(fs, "Level {}", level_info.user_num()).unwrap();
                Text::with_text_style(
                    &fs,
                    Point::new(
                        slot_rectangle.top_left.x + MARGIN,
                        slot_rectangle.center().y,
                    ),
                    MonoTextStyle::new(&FONT, Rgb565::WHITE),
                    TextStyleBuilder::new()
                        .alignment(Alignment::Left)
                        .baseline(Baseline::Middle)
                        .build(),
                )
                .draw(self.output)
                .unwrap();

                // Draw pieces
                for (i, piece) in level_info.level.all_pieces().enumerate() {
                    Sprite::new(
                        slot_rectangle.top_left
                            + Point::new(
                                slot_rectangle.size.width as i32 * 2 / 8
                                    + i as i32 * SPACE_SIZE as i32,
                                ((slot_rectangle.size.height - SPACE_SIZE) / 2) as i32,
                            ),
                        &piece.image(false),
                    )
                    .draw(self.output)
                    .unwrap();
                }

                // Draw rating
                let mut stars = level_info.rating.stars();
                stars.set_center(Point::new(
                    slot_rectangle.top_left.x + slot_rectangle.size.width as i32 * 11 / 16,
                    slot_rectangle.center().y,
                ));
                stars.draw(self.output).unwrap();

                // Draw size
                let user_size = level_info.level.user_size();
                fs.clear();
                write!(fs, "{}x{}", user_size.x, user_size.y).unwrap();
                Text::with_text_style(
                    &fs,
                    Point::new(
                        slot_rectangle.bottom_right().unwrap().x - MARGIN,
                        slot_rectangle.center().y,
                    ),
                    MonoTextStyle::new(&FONT, Rgb565::WHITE),
                    TextStyleBuilder::new()
                        .alignment(Alignment::Right)
                        .baseline(Baseline::Middle)
                        .build(),
                )
                .draw(self.output)
                .unwrap();
            }
        }

        self.output.flush();
    }

    fn update_filter(&mut self, filter: Filter, is_active: bool) {
        // TODO implement me please!
    }
}

const LEVEL_WINDOW_SIZE: usize = 7;

pub fn select_level<C: Controller, G: GameOutput>(
    controller: &mut C,
    output: &mut G,
    level_selector: &mut LevelSelector<LEVEL_WINDOW_SIZE>,
) -> GameResult<LevelInfo>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    let mut renderer = SelectRenderer::new(output);
    level_selector.render(&mut renderer);

    loop {
        let action = match controller.wait_for_action()? {
            ControlAction::Move(dir) => match dir {
                ControlDirection::Up => Action::ChangeActiveLevel(Direction::Previous),
                ControlDirection::Down => Action::ChangeActiveLevel(Direction::Next),
                ControlDirection::Left => Action::ChangeActiveFilter(Direction::Previous),
                ControlDirection::Right => Action::ChangeActiveFilter(Direction::Next),
            },
            ControlAction::A | ControlAction::Start => match level_selector.active_level_info() {
                Some(level_info) => return GameResult::Continue(level_info),
                None => continue,
            },
            _ => {
                continue;
            }
        };

        if let Some(changed) = level_selector.execute_action(action) {
            changed.render(&mut renderer);
        }
    }
}
