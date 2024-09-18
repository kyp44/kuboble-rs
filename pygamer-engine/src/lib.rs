#![no_std]
#![feature(try_trait_v2)]
#![feature(never_type)]

use core::ops::{ControlFlow, FromResidual};

use assets::stars::STAR_SIZE;
use derive_new::new;
use embedded_graphics::{
    mono_font::MonoFont, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};
use embedded_sprites::{image::Image, sprite::Sprite};
use kuboble_core::{
    level_run::Direction,
    level_select::{Action, LevelProgress, LevelSelector},
    LevelRating, Piece, Vector,
};
use level_run::play_level;
use level_select::select_level;

mod level_run;
mod level_select;

const FONT: MonoFont = embedded_graphics::mono_font::ascii::FONT_5X8;

const SPACE_SIZE: u32 = 14;
static SPACE_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(SPACE_SIZE, SPACE_SIZE));

mod assets {
    use super::*;
    use embedded_sprites::{image::Image, include_image};

    pub mod spaces {
        use super::*;

        #[include_image]
        pub const WALL: Image<Rgb565> = "assets/spaces/wall.png";
        #[include_image]
        pub const FREE: Image<Rgb565> = "assets/spaces/free.png";
    }

    pub mod pieces {
        use super::*;

        #[include_image]
        pub const GREEN: Image<Rgb565> = "assets/pieces/green.png";
        #[include_image]
        pub const GREEN_ACTIVE: Image<Rgb565> = "assets/pieces/green_active.png";
        #[include_image]
        pub const ORANGE: Image<Rgb565> = "assets/pieces/orange.png";
        #[include_image]
        pub const ORANGE_ACTIVE: Image<Rgb565> = "assets/pieces/orange_active.png";
        #[include_image]
        pub const BLUE: Image<Rgb565> = "assets/pieces/blue.png";
        #[include_image]
        pub const BLUE_ACTIVE: Image<Rgb565> = "assets/pieces/blue_active.png";
    }

    pub mod stars {
        use super::*;

        // Annoyingly, embedded_sprites::image::Image has no way to get the image size.
        pub static STAR_SIZE: Size = Size::new(11, 10);

        #[include_image]
        pub const STAR_ACTIVE: Image<Rgb565> = "assets/stars/star_active.png";
        #[include_image]
        pub const STAR_INACTIVE: Image<Rgb565> = "assets/stars/star_inactive.png";
    }
}

trait VectorExt {
    fn into_point(self) -> Point;
}
impl<T: Into<i32>> VectorExt for Vector<T> {
    fn into_point(self) -> Point {
        Point::new(self.x.into(), self.y.into())
    }
}

trait PieceExt {
    fn display_color(&self) -> Rgb565;
    fn image(&self, is_active: bool) -> Image<Rgb565>;
}
impl PieceExt for Piece {
    fn display_color(&self) -> Rgb565 {
        match self {
            Piece::Green => Rgb565::CSS_FOREST_GREEN,
            Piece::Orange => Rgb565::CSS_ORANGE,
            Piece::Blue => Rgb565::BLUE,
        }
    }

    fn image(&self, is_active: bool) -> Image<Rgb565> {
        match self {
            Piece::Green => {
                if is_active {
                    assets::pieces::GREEN_ACTIVE
                } else {
                    assets::pieces::GREEN
                }
            }
            Piece::Orange => {
                if is_active {
                    assets::pieces::ORANGE_ACTIVE
                } else {
                    assets::pieces::ORANGE
                }
            }
            Piece::Blue => {
                if is_active {
                    assets::pieces::BLUE_ACTIVE
                } else {
                    assets::pieces::BLUE
                }
            }
        }
    }
}

struct Stars {
    upper_left: Point,
    num_active: u8,
    num_stars: u8,
}
impl Stars {
    pub fn new(num_active: u8, num_stars: u8) -> Self {
        Self {
            upper_left: Point::zero(),
            num_active,
            num_stars,
        }
    }
}
impl Stars {
    pub fn set_center(&mut self, center: Point) {
        self.upper_left = Point::zero();
        self.upper_left = center - self.bounding_box().center();
    }
}
impl Dimensions for Stars {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            self.upper_left,
            Size::new(STAR_SIZE.width * self.num_stars as u32, STAR_SIZE.height),
        )
    }
}
impl Drawable for Stars {
    type Color = Rgb565;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for i in 0..self.num_active {
            Sprite::new(
                self.upper_left + Point::new(i as i32 * STAR_SIZE.width as i32, 0),
                &assets::stars::STAR_ACTIVE,
            )
            .draw(target)?
        }
        for i in self.num_active..self.num_stars {
            Sprite::new(
                self.upper_left + Point::new(i as i32 * STAR_SIZE.width as i32, 0),
                &assets::stars::STAR_INACTIVE,
            )
            .draw(target)?
        }

        Ok(())
    }
}

trait LevelRatingExt {
    fn stars(&self) -> Stars;
}
impl LevelRatingExt for LevelRating {
    fn stars(&self) -> Stars {
        Stars::new(self.num_stars(), Self::maximum_possible().num_stars())
    }
}

pub enum GameResult<T> {
    Exit,
    Continue(T),
}
impl<T> FromResidual for GameResult<T> {
    fn from_residual(_residual: <Self as core::ops::Try>::Residual) -> Self {
        Self::Exit
    }
}
impl<T> core::ops::Try for GameResult<T> {
    type Output = T;
    type Residual = ();

    fn from_output(output: Self::Output) -> Self {
        Self::Continue(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            GameResult::Exit => ControlFlow::Break(()),
            GameResult::Continue(v) => ControlFlow::Continue(v),
        }
    }
}

pub enum ControlAction {
    Move(Direction),
    A,
    B,
    Start,
    Select,
}

pub trait Controller {
    fn wait_for_action(&mut self) -> GameResult<ControlAction>;
    fn wait_for_proceed(&mut self) -> GameResult<()> {
        loop {
            match self.wait_for_action()? {
                ControlAction::A | ControlAction::Start => break GameResult::Continue(()),
                _ => {}
            }
        }
    }
}

pub trait GameDisplay: DrawTarget<Color = Rgb565> + OriginDimensions {
    fn flush(&mut self);
}

pub trait GameIndicator {
    fn indicate_active_piece(&mut self, piece: Piece);
    fn indicate_win_rating(&mut self, rating: LevelRating);
    fn indicate_nothing(&mut self);
}

pub trait GameOutput: GameDisplay + GameIndicator {
    // Slide speed in terms of pixel step size
    const SLIDE_SPEED: i32;

    // TODO: Just temporary
    fn print_test(&mut self, text: &str)
    where
        Self: Sized,
        <Self as DrawTarget>::Error: core::fmt::Debug,
    {
        embedded_graphics::text::Text::with_text_style(
            text,
            Point::zero(),
            embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                .font(&FONT)
                .text_color(Rgb565::WHITE)
                .background_color(Rgb565::BLACK)
                .build(),
            embedded_graphics::text::TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Left)
                .baseline(embedded_graphics::text::Baseline::Top)
                .build(),
        )
        .draw(self)
        .unwrap();
        self.flush();
    }
}

pub fn run_game<C: Controller, G: GameOutput>(
    mut controller: C,
    mut output: G,
    level_progress: &mut LevelProgress,
) -> GameResult<!>
where
    <G as DrawTarget>::Error: core::fmt::Debug,
{
    let mut level_selector = LevelSelector::new(level_progress);

    loop {
        let level_info = select_level(&mut controller, &mut output, &mut level_selector)?;

        if let Some(level_status) = play_level(&mut controller, &mut output, &level_info)? {
            level_selector.execute_action(Action::ActiveLevelCompleted(level_status));
        }
    }
}
