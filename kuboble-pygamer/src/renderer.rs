use core::fmt::Write;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    Circle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, StyledDrawable,
};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::{pixelcolor::Rgb565, primitives::PrimitiveStyle};
use heapless::String;
use kuboble_core::{BoardRenderer, Level, LevelRating, Piece, PieceSlid, Space, Vector};
use pygamer::gpio::v2::PA15;
use pygamer::gpio::Pin;
use pygamer::hal::hal::digital::v1_compat::OldOutputPin;
use pygamer::pac::TC3;
use pygamer::timer::TimerCounter;
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

type NeoPixels = Ws2812<TimerCounter<TC3>, OldOutputPin<Pin<PA15, Output<PushPull>>>>;

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
        }
    }

    fn neopixel_color(&self) -> RGB<u8> {
        match self {
            Piece::Green => RGB::new(0, 255, 0),
            Piece::Orange => RGB::new(255, 0, 0),
        }
    }
}

trait TextExt<D: DrawTarget> {
    // Draws the text, clearing the background first.
    fn draw_clear(&self, target: &mut D) -> Result<Point, D::Error>;
}
impl<S: TextRenderer<Color = Rgb565>, D: DrawTarget<Color = Rgb565>> TextExt<D> for Text<'_, S> {
    fn draw_clear(&self, target: &mut D) -> Result<Point, D::Error> {
        //D: DrawTarget<Color = Self::Color>

        // First clear the background
        self.character_style
            .measure_string(self.text, self.position, self.text_style.baseline)
            .bounding_box
            .draw_styled(&PrimitiveStyle::with_fill(Rgb565::BLACK), target)?;

        // Draw the text
        self.draw(target)
    }
}

pub struct LevelRenderer<'a> {
    display: &'a mut DisplayDriver,
    neopixels: &'a mut NeoPixels,
    // TODO Should precalculate positions so we do not need this eventually.
    level: &'a Level,
    board_origin: Point,
}
impl<'a> LevelRenderer<'a> {
    pub fn new(
        display: &'a mut DisplayDriver,
        neopixels: &'a mut NeoPixels,
        level: &'a Level,
    ) -> Self {
        display.clear(Rgb565::BLACK).unwrap();

        let screen_center = Rectangle::new(Point::zero(), display.size()).center();

        // Draw level number
        let mut fs: String<9> = String::new();
        // TODO: get actual level number in here!
        write!(fs, "Level {}", 1).unwrap();

        Text::with_text_style(
            &fs,
            Point::new(screen_center.x, 0),
            MonoTextStyle::new(&FONT, Rgb565::WHITE),
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(display)
        .unwrap();

        Self {
            display,
            neopixels,
            level,
            board_origin: screen_center - level.size.into_point() * (SPACE_SIZE as i32 / 2),
        }
    }

    fn board_position(&self, board_position: Vector<u8>) -> Point {
        self.board_origin + board_position.into_point() * SPACE_SIZE as i32
    }

    fn draw_num_moves(&mut self, num_moves: u8, alert: bool) {
        let mut fs: String<12> = String::new();
        write!(fs, "Moves: {}  ", num_moves).unwrap();

        Text::with_text_style(
            &fs,
            Point::new(0, self.display.size().height as i32 - 1),
            MonoTextStyle::new(&FONT, if alert { Rgb565::RED } else { Rgb565::WHITE }),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Bottom)
                .build(),
        )
        .draw_clear(self.display)
        .unwrap();
    }

    fn set_active_piece(&mut self, piece: Piece) {
        self.neopixels
            .write([RGB::new(0, 255, 0)].into_iter())
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
        .draw_clear(self.display)
        .unwrap();
    }
}
impl BoardRenderer for LevelRenderer<'_> {
    fn draw_space(&mut self, board_position: Vector<u8>, space: Space) {
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
            .translate(self.board_position(board_position))
            .into_styled(style)
            .draw(self.display)
            .unwrap();
    }

    fn draw_piece(&mut self, board_position: Vector<u8>, piece: Piece, is_active: bool) {
        // TODO: Is there a sprite system available to make this easier? May not be needed
        Circle::new(self.board_position(board_position), SPACE_SIZE)
            .into_styled(PrimitiveStyle::with_fill(piece.display_color()))
            .draw(self.display)
            .unwrap();

        if is_active {
            Circle::with_center(
                self.board_position(board_position) + SPACE_RECT.center(),
                SPACE_SIZE / 2,
            )
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(self.display)
            .unwrap();

            self.set_active_piece(piece);
        }
    }

    fn slide_piece(&mut self, piece_slid: PieceSlid, is_active: bool) {
        // TODO: Animate this with constant slide time? Observe how the web version does it
        self.draw_space(
            piece_slid.starting_position,
            self.level.get_space(piece_slid.starting_position),
        );
        self.draw_piece(
            piece_slid.starting_position
                + piece_slid.muv.direction.as_vector() * piece_slid.distance.try_into().unwrap(),
            piece_slid.muv.piece,
            is_active,
        );
    }

    fn update_num_moves(&mut self, num_moves: u8, at_maximum: bool) {
        self.draw_num_moves(num_moves, at_maximum);
    }

    fn update_goal(&mut self, goal: u8) {
        let mut fs: String<9> = String::new();
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

    fn notify_win(&mut self, rating: LevelRating) {
        //todo!()
    }
}
