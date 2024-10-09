use core::iter::{repeat, repeat_n};

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;
use kuboble_core::{LevelRating, Piece};
use pygamer::{
    hal::{
        gpio::{Output, Pin, PushPull, PA15},
        prelude::*,
        timer::TimerCounter4,
    },
    pac::Tc4,
    TftDc, TftReset, TftSpi,
};
use pygamer_engine::{BufferedDisplay, GameDisplay, GameIndicator, GameOutput};
use rtic_monotonics::rtic_time::embedded_hal::delay::DelayNs;
use rtic_monotonics::systick::prelude::*;
use rtic_monotonics::Monotonic;
use smart_leds::{SmartLedsWrite, RGB};

use crate::Mono;

pub type DisplayDriver = st7735_lcd::ST7735<TftSpi, TftDc, TftReset>;

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

pub async fn neopixels_test(mut neopixels: NeoPixels) -> ! {
    loop {
        let colors = [Piece::Green.neopixel_color(), RGB::default()];

        neopixels.write(colors.into_iter().cycle().take(5)).unwrap();
        Mono::delay(333.millis()).await;

        let colors = [Piece::Orange.neopixel_color(), RGB::default()];

        neopixels.write(colors.into_iter().cycle().take(5)).unwrap();
        Mono::delay(333.millis()).await;

        let colors = [Piece::Blue.neopixel_color(), RGB::default()];

        neopixels.write(colors.into_iter().cycle().take(5)).unwrap();
        Mono::delay(333.millis()).await;
    }
}

pub async fn display_test(mut display: DisplayDriver) -> ! {
    display.clear(Rgb565::WHITE).unwrap();
    loop {
        embedded_graphics::primitives::Rectangle::new(Point::zero(), Size::new(100, 100))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_DARK_BLUE))
            .draw(&mut display)
            .unwrap();
        Mono::delay(1.secs()).await;

        embedded_graphics::primitives::Rectangle::new(Point::zero(), Size::new(100, 100))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_DARK_RED))
            .draw(&mut display)
            .unwrap();
        Mono::delay(1.secs()).await;
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
