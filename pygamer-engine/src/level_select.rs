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
    level_select::{render::LevelSelectRenderer, Filter, LevelSlotInfo, LevelStatus},
    levels::MAX_STRIP_SIZE,
    Level, Piece, Space, Vector,
};

const SLOT_HEIGHT: u32 = 14;
const SLOT_WIDTH: u32 = 100;
const WINDOW_SIZE: usize = 5;
static SLOT_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(SLOT_WIDTH, SLOT_HEIGHT));

pub struct SelectRenderer<'a, G> {
    output: &'a mut G,
    slots_origin: Point,
}
impl<'a, G: GameOutput> SelectRenderer<'a, G>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    pub fn new(output: &'a mut G) -> Self {
        output.clear(Rgb565::BLACK).unwrap();

        let display_center = Rectangle::new(Point::zero(), output.size()).center();

        Self {
            output,
            slots_origin: display_center
                - Point::new(SLOT_WIDTH as i32, SLOT_HEIGHT as i32 * WINDOW_SIZE as i32) / 2,
        }
    }

    fn slot_rectangle(&self, position: u8) -> Rectangle {
        SLOT_RECT.translate(self.slots_origin + Point::new(0, position as i32 * SLOT_HEIGHT as i32))
    }

    fn fill_slot(&mut self, position: u8, stroke: Option<Rgb565>, background: Rgb565) {
        let mut style = PrimitiveStyleBuilder::new().fill_color(background).build();

        if let Some(color) = stroke {
            style.stroke_color = Some(color);
            style.stroke_width = 2;
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

        let mut fs: ArrayString<12> = ArrayString::new();

        // TODO Draw rectangle with color depending on active

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
                        Rgb565::CSS_DIM_GRAY
                    }),
                    if *is_active {
                        Rgb565::CSS_DARK_GREEN
                    } else {
                        Rgb565::BLACK
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
            }
        }

        self.output.flush();
    }

    fn update_filter(&mut self, filter: Filter, is_active: bool) {
        // TODO implement me please!
    }
}
