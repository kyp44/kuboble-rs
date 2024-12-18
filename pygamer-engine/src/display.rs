use core::convert::Infallible;

use embedded_graphics::mono_font;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{prelude::*, text};
use embedded_graphics_framebuf::FrameBuf;

pub const DISPLAY_SIZE: Size = Size::new(160, 128);
pub const FONT: mono_font::MonoFont = embedded_graphics::mono_font::ascii::FONT_5X8;

// TODO: No crate that does this exists
pub struct DisplayTextStyle<C> {
    /// Location of the text on the screen.
    position: Point,
    /// Optional box size for character wrapping.
    char_box_size: Option<Size>,
    character_style: mono_font::MonoTextStyle<'static, C>,
    text_style: text::TextStyle,
}
impl<C> DisplayTextStyle<C> {
    pub fn new(
        position: Point,
        box_size: Option<Size>,
        character_style: mono_font::MonoTextStyle<'static, C>,
        text_style: text::TextStyle,
    ) -> Self {
        Self {
            position,
            char_box_size: box_size.map(|s| {
                Size::new(
                    s.width / character_style.font.character_size.width,
                    s.height / character_style.font.character_size.height,
                )
            }),

            character_style,
            text_style,
        }
    }
}

pub struct DisplayWriter<'a, D, C> {
    /// The display to which to write
    display: &'a mut D,
    /// The text style
    text_style: &'a DisplayTextStyle<C>,
    /// Current cursor location in character space.
    char_cursor: Point,
}
impl<'a, D, C> DisplayWriter<'a, D, C> {
    pub fn new(display: &'a mut D, text_style: &'a DisplayTextStyle<C>) -> Self {
        Self {
            display,
            text_style,
            char_cursor: Point::zero(),
        }
    }
}
impl<D: DrawTarget<Color = C>, C: PixelColor> core::fmt::Write for DisplayWriter<'_, D, C> {
    fn write_str(&mut self, mut s: &str) -> core::fmt::Result {
        let style = self.text_style;

        loop {
            if s.is_empty() {
                break;
            }
            if let Some(cs) = style.char_box_size
                && self.char_cursor.y >= cs.height as i32
            {
                break;
            }

            let (line_s, rem_s) = match style.char_box_size {
                Some(cs) => {
                    // Iterator of character indices in the current string
                    let mut char_idxs = s.char_indices();

                    // Advance by the number of characters left on the current line
                    let _ = char_idxs.advance_by(cs.width as usize - self.char_cursor.x as usize);

                    let idx = char_idxs.next().map(|t| t.0).unwrap_or(s.len());
                    s.split_at_checked(idx).ok_or(core::fmt::Error)?
                }
                None => (s, ""),
            };

            text::Text::with_text_style(
                line_s,
                style.position
                    + Point::new(
                        self.char_cursor.x * style.character_style.font.character_size.width as i32,
                        self.char_cursor.y
                            * style.character_style.font.character_size.height as i32,
                    ),
                style.character_style,
                style.text_style,
            )
            .draw(self.display)
            .map_err(|_| core::fmt::Error)?;

            // Update cursor
            self.char_cursor.x += line_s.len() as i32;

            // Advance to the next line if applicable
            if let Some(cs) = style.char_box_size
                && self.char_cursor.x >= cs.width as i32
            {
                self.char_cursor.x = 0;
                self.char_cursor.y += 1;
            }

            s = rem_s;
        }

        Ok(())
    }
}

/// A [`DrawTarget`] that is just a frame buffer in memory for the
/// entire PyGamer display.
///
/// This also implements [`Drawable`] so that it can be rendered to the
/// actual display.
pub struct BufferedDisplay {
    frame_buffer: [Rgb565; DISPLAY_SIZE.width as usize * DISPLAY_SIZE.height as usize],
}
impl Drawable for BufferedDisplay {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.fill_contiguous(
            &Rectangle::new(Point::zero(), DISPLAY_SIZE),
            self.frame_buffer.iter().copied(),
        )
    }
}
impl Default for BufferedDisplay {
    fn default() -> Self {
        Self {
            frame_buffer: [Rgb565::default();
                DISPLAY_SIZE.width as usize * DISPLAY_SIZE.height as usize],
        }
    }
}
impl OriginDimensions for BufferedDisplay {
    fn size(&self) -> Size {
        DISPLAY_SIZE
    }
}
impl DrawTarget for BufferedDisplay {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        FrameBuf::new(
            &mut self.frame_buffer,
            DISPLAY_SIZE.width as usize,
            DISPLAY_SIZE.height as usize,
        )
        .draw_iter(pixels)
    }
}
