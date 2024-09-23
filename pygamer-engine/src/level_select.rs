use crate::{
    assets::{pieces::SMALL_SIZE, stars::STAR_SIZE},
    ControlAction, Controller, GameOutput, GameResult, LevelRatingExt, PieceExt, Stars,
    DISPLAY_SIZE, FONT,
};
use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::{
    geometry::AnchorPoint,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_sprites::sprite::Sprite;
use kuboble_core::{
    level_run::Direction as ControlDirection,
    level_select::{
        render::LevelSelectRenderer, Action, Direction, Filter, LevelInfo, LevelSelector,
        LevelSlotInfo,
    },
    BufferedRenderer, LevelRating,
};

const LEVEL_WINDOW_SIZE: usize = 7;

const SLOT_HEIGHT: u32 = 14;

static SLOT_RECT: Rectangle = Rectangle::new(
    Point::new(0, SLOT_HEIGHT as i32 + 1),
    Size::new(DISPLAY_SIZE.width, SLOT_HEIGHT),
);

const MARGIN: i32 = 3;
const FILTER_CENTER_Y: i32 = SLOT_HEIGHT as i32 / 2 - 1;
const FILTER_GAP: i32 = 6;

fn rect_style(is_active: Option<bool>) -> PrimitiveStyle<Rgb565> {
    match is_active {
        Some(is_active) => PrimitiveStyleBuilder::new()
            .fill_color(if is_active {
                Rgb565::CSS_DARK_GREEN
            } else {
                Rgb565::CSS_DARK_SLATE_GRAY
            })
            .stroke_color(if is_active {
                Rgb565::WHITE
            } else {
                Rgb565::CSS_SLATE_GRAY
            })
            .stroke_width(1)
            .build(),
        None => PrimitiveStyle::with_fill(Rgb565::BLACK),
    }
}

pub struct SelectRenderer<'a, G> {
    output: &'a mut G,
    all_text: Text<'static, MonoTextStyle<'static, Rgb565>>,
    no_stars: Stars,
}
impl<'a, G: GameOutput> SelectRenderer<'a, G>
where
    G::Error: core::fmt::Debug,
{
    pub fn new(output: &'a mut G) -> Self {
        output.clear(Rgb565::BLACK).unwrap();

        // Filter label
        Text::with_text_style(
            "Filter:",
            Point::new(MARGIN, FILTER_CENTER_Y),
            MonoTextStyle::new(&FONT, Rgb565::WHITE),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(output)
        .unwrap();

        // The all levels filter text, needed now to have the bounding rectangle
        let all_text = Text::with_text_style(
            "ALL",
            Point::new(SLOT_RECT.center().x + 1, FILTER_CENTER_Y),
            MonoTextStyle::new(&FONT, Rgb565::YELLOW),
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Middle)
                .build(),
        );

        // The incomplete level stars, needed now to have the bounding rectangle.
        let no_stars = Stars::new(
            Point::new(
                all_text.bounding_box().bottom_right().unwrap().x + FILTER_GAP,
                FILTER_CENTER_Y - STAR_SIZE.height as i32 / 2 + 1,
            ),
            0,
            2,
        );

        Self {
            output,
            all_text,
            no_stars,
        }
    }

    fn slot_rectangle(position: u8) -> Rectangle {
        SLOT_RECT.translate(Point::new(0, position as i32 * SLOT_HEIGHT as i32))
    }

    fn fill_slot(&mut self, position: u8, is_active: Option<bool>) {
        Self::slot_rectangle(position)
            .into_styled(rect_style(is_active))
            .draw(self.output)
            .unwrap();
    }

    fn filter_stars_point(&self, n: u8) -> Point {
        Point::new(
            self.all_text.bounding_box().bottom_right().unwrap().x
                + FILTER_GAP
                + (self.no_stars.bounding_box().size.width as i32 + FILTER_GAP) * n as i32,
            self.no_stars.bounding_box().top_left.y,
        )
    }
}
impl<G: GameOutput> BufferedRenderer for SelectRenderer<'_, G> {
    fn flush(&mut self) {
        self.output.flush();
    }
}
impl<G: GameOutput> LevelSelectRenderer for SelectRenderer<'_, G>
where
    G::Error: core::fmt::Debug,
{
    fn draw_level_slot(&mut self, level_slot_info: &LevelSlotInfo) {
        let mut fs: ArrayString<10> = ArrayString::new();

        match level_slot_info {
            LevelSlotInfo::Empty(p) => self.fill_slot(*p, None),
            LevelSlotInfo::Level {
                level_info,
                position,
                is_active,
            } => {
                self.fill_slot(*position, Some(*is_active));

                let slot_rectangle = Self::slot_rectangle(*position);

                const SECTION_GAP: i32 = 5;

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
                let pieces_x: i32 = slot_rectangle.top_left.x
                    + MARGIN
                    + FONT.character_size.width as i32 * 9
                    + FONT.character_spacing as i32 * 8
                    + SECTION_GAP;
                for (i, piece) in level_info.level.all_pieces().enumerate() {
                    Sprite::new(
                        slot_rectangle.top_left
                            + Point::new(
                                pieces_x + i as i32 * (SMALL_SIZE.width as i32 + 1),
                                (slot_rectangle.size.height - SMALL_SIZE.height) as i32 / 2,
                            ),
                        &piece.image_small(),
                    )
                    .draw(self.output)
                    .unwrap();
                }

                // Draw rating
                let rating_x: i32 = pieces_x + SMALL_SIZE.width as i32 * 3 + 2 + SECTION_GAP;
                level_info
                    .rating
                    .stars(Point::new(
                        rating_x,
                        slot_rectangle.center().y - STAR_SIZE.height as i32 / 2 + 1,
                    ))
                    .draw(self.output)
                    .unwrap();

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
    }

    fn update_filter(&mut self, filter: Filter, is_active: bool) {
        fn draw_selected_box<G: DrawTarget<Color = Rgb565>>(
            output: &mut G,
            bounding_box: Rectangle,
            is_active: bool,
        ) where
            G::Error: core::fmt::Debug,
        {
            bounding_box
                .resized(bounding_box.size + Size::new(4, 4), AnchorPoint::Center)
                .into_styled(rect_style(is_active.then_some(true)))
                .draw(output)
                .unwrap();
        }

        match filter {
            Filter::All => {
                draw_selected_box(self.output, self.all_text.bounding_box(), is_active);
                self.all_text.draw(self.output).unwrap();
            }
            Filter::Incomplete => {
                draw_selected_box(self.output, self.no_stars.bounding_box(), is_active);
                self.no_stars.draw(self.output).unwrap();
            }
            Filter::PartiallyComplete => {
                let stars = Stars::new(self.filter_stars_point(1), 1, 2);
                draw_selected_box(self.output, stars.bounding_box(), is_active);
                stars.draw(self.output).unwrap();
            }
            Filter::Optimal => {
                let stars = Stars::new(self.filter_stars_point(2), 2, 2);
                draw_selected_box(self.output, stars.bounding_box(), is_active);
                stars.draw(self.output).unwrap();
            }
        };
    }

    fn update_num_locked(&mut self, num_locked: u16) {
        self.draw_level_slot(&LevelSlotInfo::Empty(LEVEL_WINDOW_SIZE as u8));

        let mut fs: ArrayString<14> = ArrayString::new();
        write!(fs, "{num_locked} to unlock").unwrap();

        Text::with_text_style(
            &fs,
            Self::slot_rectangle(LEVEL_WINDOW_SIZE as u8).center(),
            MonoTextStyle::new(&FONT, Rgb565::WHITE),
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(self.output)
        .unwrap();
    }

    fn update_active_rating(&mut self, rating: Option<LevelRating>) {
        match rating {
            Some(r) => self.output.indicate_win_rating(r),
            None => self.output.indicate_nothing(),
        }
    }
}

pub fn select_level<C: Controller, G: GameOutput>(
    controller: &mut C,
    output: &mut G,
    level_selector: &mut LevelSelector<LEVEL_WINDOW_SIZE>,
) -> GameResult<LevelInfo>
where
    G::Error: core::fmt::Debug,
{
    let mut renderer = SelectRenderer::new(output);
    level_selector.render(&mut renderer);

    loop {
        let action = match controller.wait_for_action()? {
            ControlAction::Move(dir) => match dir {
                ControlDirection::Up => Action::ChangeActiveLevel(Direction::Previous),
                ControlDirection::Down => Action::ChangeActiveLevel(Direction::Next),
                // TODO left and right to page up/down
                //ControlDirection::Left => Action::ChangeActiveFilter(Direction::Previous),
                //ControlDirection::Right => Action::ChangeActiveFilter(Direction::Next),
                _ => continue,
            },
            ControlAction::Select => Action::ChangeActiveFilter(Direction::Next),
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
