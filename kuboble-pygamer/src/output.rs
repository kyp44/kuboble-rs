use derive_new::new;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use kuboble_core::{LevelRating, Piece};
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
use pygamer_engine::{GameDisplay, GameIndicator, GameOutput};
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

#[derive(new)]
pub struct PyGamerOutput<T> {
    display: DisplayDriver,
    neopixels: NeoPixels<T>,
}
impl<T: CountDown + Periodic> GameIndicator for PyGamerOutput<T> {
    fn indicate_active_piece(&mut self, piece: Piece) {
        let colors = [piece.neopixel_color(), RGB::new(0, 0, 0)];

        self.neopixels
            .write(colors.into_iter().cycle().take(5))
            .unwrap();
    }

    fn indicate_win_rating(&mut self, rating: LevelRating) {
        // TODO: Implement!
    }

    fn indicate_nothing(&mut self) {
        // TODO: Implement!
    }
}
impl<T> OriginDimensions for PyGamerOutput<T> {
    #[inline]
    fn size(&self) -> Size {
        self.display.size()
    }
}
impl<T> DrawTarget for PyGamerOutput<T> {
    type Color = Rgb565;

    type Error = <DisplayDriver as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels)
    }
}
impl<T> GameDisplay for PyGamerOutput<T> {
    fn flush(&mut self) {
        // Display writes are immediate so do nothing here
    }
}
impl<T: CountDown + Periodic> GameOutput for PyGamerOutput<T> {
    const SLIDE_SPEED: i32 = 2;
}
