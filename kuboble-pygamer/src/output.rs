use core::iter::{repeat, repeat_n};

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use kuboble_core::{LevelRating, Piece};
use pygamer::{
    hal::{
        gpio::{Output, Pin, PushPull, PA15},
        timer::TimerCounter4,
    },
    pins::DisplayDriver,
};
use pygamer_engine::{BufferedDisplay, GameDisplay, GameIndicator, GameOutput};
use smart_leds::{SmartLedsWrite, RGB};

pub type NeoPixels = ws2812_timer_delay::Ws2812<TimerCounter4, Pin<PA15, Output<PushPull>>>;

trait PieceExt {
    fn neopixel_color(&self) -> RGB<u8>;
}
impl PieceExt for Piece {
    fn neopixel_color(&self) -> RGB<u8> {
        match self {
            Piece::Green => RGB::new(0, 5, 0),
            Piece::Orange => RGB::new(5, 3, 0),
            Piece::Blue => RGB::new(0, 0, 10),
        }
    }
}

const STAR_COLOR: RGB<u8> = RGB::new(4, 4, 0);

pub struct PyGamerOutput {
    display: DisplayDriver,
    buffer: BufferedDisplay,
    neopixels: NeoPixels,
}
impl PyGamerOutput {
    pub fn new(display: DisplayDriver, neopixels: NeoPixels) -> Self {
        Self {
            display,
            buffer: BufferedDisplay::default(),
            neopixels,
        }
    }
}
impl GameIndicator for PyGamerOutput {
    fn indicate_active_piece(&mut self, piece: Piece) {
        let colors = [piece.neopixel_color(), RGB::default()];

        self.neopixels
            .write(colors.into_iter().cycle().take(5))
            .unwrap();
    }

    fn indicate_win_rating(&mut self, rating: LevelRating) {
        self.neopixels
            .write(
                repeat_n(STAR_COLOR, rating.num_stars() as usize)
                    .chain(repeat(RGB::default()))
                    .take(5),
            )
            .unwrap();
    }

    fn indicate_nothing(&mut self) {
        // NOTE: Due to the janky timing on the neopixels, using RGB::default() here does not produce black!
        self.neopixels
            .write(repeat_n(RGB::new(0, 0, 0), 5))
            .unwrap();
    }
}
impl OriginDimensions for PyGamerOutput {
    #[inline]
    fn size(&self) -> Size {
        self.buffer.size()
    }
}
impl DrawTarget for PyGamerOutput {
    type Color = Rgb565;

    type Error = <BufferedDisplay as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.buffer.draw_iter(pixels)
    }
}
impl GameDisplay for PyGamerOutput {
    fn flush(&mut self) {
        self.buffer.draw(&mut self.display).unwrap()
    }
}
impl GameOutput for PyGamerOutput {
    const SLIDE_SPEED: i32 = 14;
}
