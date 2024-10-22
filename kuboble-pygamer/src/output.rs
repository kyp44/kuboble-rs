use core::iter::{repeat, repeat_n};

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;
use kuboble_core::{LevelRating, Piece};
use pygamer::{
    hal::{
        fugit::ExtU64,
        gpio::{Output, Pin, PushPull, PA15},
        prelude::*,
        timer::TimerCounter4,
    },
    TftDc, TftReset, TftSpi,
};
use pygamer_engine::{BufferedDisplay, GameDisplay, GameIndicator, GameOutput};
use smart_leds::{SmartLedsWrite, RGB};

use crate::Mono;

pub type DisplayDriver = st7735_lcd::ST7735<TftSpi, TftDc, TftReset>;

pub type NeoPixels = ws2812_timer_delay::Ws2812<TimerCounter4, Pin<PA15, Output<PushPull>>>;

pub trait PieceExt {
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

trait NeoPixelsExt {
    fn set(&mut self, iter: impl Iterator<Item = RGB<u8>>);
}
impl NeoPixelsExt for NeoPixels {
    fn set(&mut self, iter: impl Iterator<Item = RGB<u8>>) {
        // Need to disable interrupts here because interrupts can wreck the timing
        critical_section::with(|_| {
            self.write(iter).unwrap();
        })
    }
}

pub async fn neopixels_test(mut neopixels: NeoPixels) -> ! {
    #[inline]
    fn test_colors(color: RGB<u8>) -> impl Iterator<Item = RGB<u8>> {
        [color; 5].into_iter()
        //colors.into_iter().cycle().take(5))
    }

    loop {
        //const DELAY_MS: u64 = 750;
        const DELAY_MS: u64 = 1000;

        neopixels.set(test_colors(Piece::Green.neopixel_color()));
        Mono::delay(DELAY_MS.millis()).await;

        let colors = [Piece::Orange.neopixel_color(); 5];

        neopixels.set(test_colors(Piece::Orange.neopixel_color()));
        Mono::delay(DELAY_MS.millis()).await;

        let colors = [Piece::Blue.neopixel_color(); 5];

        neopixels.set(test_colors(Piece::Blue.neopixel_color()));
        Mono::delay(DELAY_MS.millis()).await;
    }
}

pub async fn display_test(mut display: DisplayDriver) -> ! {
    const DELAY_MS: u64 = 1000;

    display.clear(Rgb565::WHITE).unwrap();
    loop {
        embedded_graphics::primitives::Rectangle::new(Point::zero(), Size::new(100, 100))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_DARK_BLUE))
            .draw(&mut display)
            .unwrap();
        Mono::delay(DELAY_MS.millis()).await;

        embedded_graphics::primitives::Rectangle::new(Point::zero(), Size::new(100, 100))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_DARK_RED))
            .draw(&mut display)
            .unwrap();
        Mono::delay(DELAY_MS.millis()).await;
    }
}
/*
pub struct PyGamerOutput<T> {
    display: DisplayDriver,
    buffer: BufferedDisplay,
    neopixels: NeoPixels<T>,
}
impl<T> PyGamerOutput<T> {
    pub fn new(display: DisplayDriver, neopixels: NeoPixels<T>) -> Self {
        Self {
            display,
            buffer: BufferedDisplay::default(),
            neopixels,
        }
    }
}
impl<T: CountDown + Periodic> GameIndicator for PyGamerOutput<T> {
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
impl<T> OriginDimensions for PyGamerOutput<T> {
    #[inline]
    fn size(&self) -> Size {
        self.buffer.size()
    }
}
impl<T> DrawTarget for PyGamerOutput<T> {
    type Color = Rgb565;

    type Error = <BufferedDisplay as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.buffer.draw_iter(pixels)
    }
}
impl<T> GameDisplay for PyGamerOutput<T> {
    fn flush(&mut self) {
        self.buffer.draw(&mut self.display).unwrap()
    }
}
impl<T: CountDown + Periodic> GameOutput for PyGamerOutput<T> {
    const SLIDE_SPEED: i32 = 14;
}
 */
