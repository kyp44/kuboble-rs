use core::fmt::Write;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::{pixelcolor::Rgb565, primitives::PrimitiveStyle};
use embedded_graphics_framebuf::FrameBuf;
use heapless::String;
use kuboble_core::level_run::render::LevelRunRenderer;
use kuboble_core::level_run::PieceSlid;
use kuboble_core::level_select::LevelStatus;
use kuboble_core::{Level, Piece, Space, Vector};
use pygamer::gpio::v2::PA15;
use pygamer::gpio::Pin;
use pygamer::hal::hal::digital::v1_compat::OldOutputPin;
use pygamer::hal::hal::timer::{CountDown, Periodic};
use pygamer::{
    gpio::v2::{Alternate, Output, PushPull, C, PA00, PB05, PB13, PB14, PB15},
    pac::SERCOM4,
    sercom::{
        v2::{Pad1, Pad2, Pad3},
        Pad, SPIMaster4,
    },
};
use smart_leds::{SmartLedsWrite, RGB};
use ws2812_timer_delay::Ws2812;

type DisplayDriver = st7735_lcd::ST7735<
    SPIMaster4<
        Pad<SERCOM4, Pad2, Pin<PB14, Alternate<C>>>,
        Pad<SERCOM4, Pad3, Pin<PB15, Alternate<C>>>,
        Pad<SERCOM4, Pad1, Pin<PB13, Alternate<C>>>,
    >,
    Pin<PB05, Output<PushPull>>,
    Pin<PA00, Output<PushPull>>,
>;

type NeoPixels<T> = Ws2812<T, OldOutputPin<Pin<PA15, Output<PushPull>>>>;

const SPACE_SIZE: u32 = 12;
static SPACE_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(SPACE_SIZE, SPACE_SIZE));
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
    fn neopixel_color(&self) -> RGB<u8>;
}
impl PieceExt for Piece {
    fn display_color(&self) -> Rgb565 {
        match self {
            Piece::Green => Rgb565::GREEN,
            Piece::Orange => Rgb565::CSS_ORANGE,
            Piece::Blue => Rgb565::BLUE,
        }
    }

    fn neopixel_color(&self) -> RGB<u8> {
        match self {
            Piece::Green => RGB::new(0, 5, 0),
            Piece::Orange => RGB::new(5, 3, 0),
            Piece::Blue => RGB::new(0, 0, 10),
        }
    }
}

pub struct LevelRenderer<'a, T> {
    display: &'a mut DisplayDriver,
    neopixels: &'a mut NeoPixels<T>,
    level_origin: Point,
    display_center: Point,
    // This makes updating the number of moves more efficient.
    at_max_moves: bool,
}
impl<'a, T: CountDown + Periodic> LevelRenderer<'a, T> {
    pub fn new(
        display: &'a mut DisplayDriver,
        neopixels: &'a mut NeoPixels<T>,
        level: &'a Level,
    ) -> Self {
        display.clear(Rgb565::BLACK).unwrap();

        let display_center = Rectangle::new(Point::zero(), display.size()).center();

        Self {
            display,
            neopixels,
            level_origin: display_center - level.size.into_point() * (SPACE_SIZE as i32 / 2),
            display_center,
            at_max_moves: true,
        }
    }

    fn absolute_position(&self, level_position: Vector<u8>) -> Point {
        self.level_origin + level_position.into_point() * SPACE_SIZE as i32
    }

    fn set_active_piece(&mut self, piece: Piece) {
        let colors = [piece.neopixel_color(), RGB::new(0, 0, 0)];

        self.neopixels
            .write(colors.into_iter().cycle().take(5))
            .unwrap();
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
impl<T: CountDown + Periodic> LevelRunRenderer for LevelRenderer<'_, T> {
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
        // TODO: Is there a sprite system available to make this easier? May not be needed
        self.draw_space(piece_slid.starting_position, piece_slid.starting_space);
        self.draw_piece(
            piece_slid.starting_position
                + piece_slid.muv.direction.as_vector() * piece_slid.distance.try_into().unwrap(),
            piece_slid.muv.piece,
            is_active,
        );
    }

    fn update_num_moves(&mut self, num_moves: u8, at_maximum: bool) {
        let mut fs: String<12> = String::new();

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
        let mut fs: String<10> = String::new();

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
        let mut fs: String<24> = String::new();
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
