#![feature(let_chains)]

use clap::Parser;
use euclid::{Point2D, Size2D, Transform2D, Vector2D};
use grid::Grid;
use itertools::{iproduct, Itertools};
use kuboble_core::{Piece, Space};
use serde::Deserialize;
use std::{fs::File, io::BufReader, io::Write, path::PathBuf};
use strum::IntoEnumIterator;

trait SizeExt {
    fn into_rust(self) -> Size<RustLevel>;
}
impl SizeExt for Size<JsonLevel> {
    fn into_rust(self) -> Size<RustLevel> {
        // The dimensions get flipped due to rotation
        Size::new(self.height + 2, self.width + 2)
    }
}

trait PointExt {
    fn into_rust(self, size: Size<JsonLevel>) -> Point<RustLevel>;
}
impl PointExt for Point<JsonLevel> {
    fn into_rust(self, size: Size<JsonLevel>) -> Point<RustLevel> {
        // Precalculated matrix
        let transform: Transform2D<i8, JsonLevel, RustLevel> =
            Transform2D::new(0, -1, 1, 0, 1, size.width as i8);

        transform
            .transform_point(self.try_cast().unwrap())
            .try_cast()
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
struct JsonLevel {
    x: u8,
    y: u8,
    #[serde(rename = "n")]
    _n: usize,
    s: Vec<u8>,
    t: Vec<u8>,
    o: Vec<u8>,
}
impl JsonLevel {
    pub fn size(&self) -> Size<JsonLevel> {
        Size::new(self.x, self.y)
    }

    fn space_num_to_point(&self, n: u8) -> Point<JsonLevel> {
        let width = self.size().width;

        Point::new(n % width, n / width)
    }
}

trait GridExt {
    fn get_space(&self, point: Point<RustLevel>) -> Option<Space>;
    fn set_space(&mut self, point: Point<RustLevel>, space: Space);
    fn is_surrounded_by(&self, point: Point<RustLevel>, space: Space) -> bool;
}
impl GridExt for Grid<Space> {
    fn get_space(&self, point: Point<RustLevel>) -> Option<Space> {
        self.get(point.y as usize, point.x as usize).copied()
    }

    fn set_space(&mut self, point: Point<RustLevel>, space: Space) {
        *self.get_mut(point.y as usize, point.x as usize).unwrap() = space;
    }

    fn is_surrounded_by(&self, point: Point<RustLevel>, space: Space) -> bool {
        iproduct!(-1i8..=1, -1i8..=1)
            .map(|(dy, dx)| Vector2D::new(dx, dy))
            .filter(|dv| *dv != Vector2D::zero())
            .all(|dv| {
                if let Some(p) = (point.try_cast().unwrap() + dv).try_cast()
                    && let Some(s) = self.get_space(p)
                {
                    s == space
                } else {
                    true
                }
            })
    }
}

type Size<U> = Size2D<u8, U>;
type Point<U> = Point2D<u8, U>;

#[derive(Debug)]
struct RustLevel {
    level_num: u16,
    size: Size<Self>,
    spaces: Grid<Space>,
    positions: Vec<Point<Self>>,
    optimal: u8,
}
impl RustLevel {
    fn from_json_level(value: JsonLevel, level_num: u16, optimal: u8) -> Self {
        let rust_size = value.size().into_rust();

        // Initialize spaces with all walls
        let mut spaces = Grid::new(rust_size.height as usize, rust_size.width as usize);
        spaces.fill(Space::Wall);

        // Carve out internal free spaces
        for point in iproduct!(1..rust_size.height - 1, 1..rust_size.width - 1)
            .map(|(y, x)| -> Point<RustLevel> { Point::new(x, y) })
        {
            spaces.set_space(point, Space::Free);
        }

        // Fill additional walls
        for n in value.o.iter() {
            spaces.set_space(
                value.space_num_to_point(*n).into_rust(value.size()),
                Space::Wall,
            );
        }

        // Remove extraneous walls by voiding wall spaces surrounded by other walls (or the edge of the level)
        let void_points = iproduct!(0..spaces.size().0, 0..spaces.size().1)
            .map(|(y, x)| Point::new(x as u8, y as u8))
            .filter(|p| {
                spaces.get_space(*p).unwrap() == Space::Wall
                    && spaces.is_surrounded_by(*p, Space::Wall)
            })
            .collect_vec();
        for point in void_points {
            spaces.set_space(point, Space::Void)
        }

        // Fill in goal spaces
        for (n, piece) in value.t.iter().zip(Piece::iter()) {
            spaces.set_space(
                value.space_num_to_point(*n).into_rust(value.size()),
                Space::Goal(piece),
            );
        }

        // Set piece positions
        let positions = value
            .s
            .iter()
            .map(|n| value.space_num_to_point(*n).into_rust(value.size()))
            .collect_vec();

        Self {
            level_num: level_num,
            size: rust_size,
            spaces,
            positions,
            optimal,
        }
    }
}
impl std::fmt::Display for RustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "// Level {}", self.level_num)?;
        writeln!(f, "level! {{")?;
        writeln!(f, "\tspaces: &[")?;
        for row in self.spaces.iter_rows() {
            writeln!(f, "\t\t\"{}\",", row.map(|s| char::from(*s)).join(""))?;
        }
        writeln!(f, "\t],")?;
        writeln!(
            f,
            "\tpositions: &[{}],",
            self.positions
                .iter()
                .map(|p| format!("Vector::new{p:?}"))
                .join(", ")
        )?;
        writeln!(f, "\toptimal: {},", self.optimal)?;
        write!(f, "}},")?;

        Ok(())
    }
}

// It would be nice to be able to use a doc string for the about but evidently there is no way
// to do this and have multiple lines:
// https://github.com/clap-rs/clap/issues/5003
#[derive(Parser, Debug)]
#[command(
    version,
    about = r#"Converts Kuboble level definitions from JSON to Rust code and prints some metrics.
    
The JSON level definition file must be in the format defined here: https://github.com/redking00/kubobleSolverJS
Unfortunately, this does not include the the optimal number of moves for each level, so these must be specified in an additional file"#
)]
struct Args {
    /// JSON level definition file to convert.
    json_level_path: PathBuf,

    /// JSON optimal number of moves file.
    json_optimal_moves: PathBuf,
}

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Parse the level definition file
    let json_levels: Vec<JsonLevel> =
        serde_json::from_reader(BufReader::new(File::open(&args.json_level_path)?))?;

    // Parse the optimal moves file
    let optimal_moves: Vec<u8> =
        serde_json::from_reader(BufReader::new(File::open(&args.json_optimal_moves)?))?;

    // Determine max number of pieces
    let max_pieces = json_levels.iter().map(|jl| jl.s.len()).max().unwrap();

    // Convert levels
    let rust_levels = json_levels
        .into_iter()
        .zip(optimal_moves)
        .enumerate()
        .map(|(idx, (jl, optimal))| RustLevel::from_json_level(jl, idx as u16 + 1, optimal))
        .collect_vec();

    // Determine other
    println!("Number of levels: {}", rust_levels.len());
    println!("Max number of pieces: {}", max_pieces);
    println!(
        "Maximum optimal number of moves: {}",
        rust_levels.iter().map(|rl| rl.optimal).max().unwrap()
    );
    let level = rust_levels.iter().max_by_key(|l| l.size.width).unwrap();
    println!(
        "Maximum height width: Level {} at {:?}",
        level.level_num, level.size
    );
    let level = rust_levels.iter().max_by_key(|l| l.size.height).unwrap();
    println!(
        "Maximum height size: Level {} at {:?}",
        level.level_num, level.size
    );

    let mut rust_file = File::create(
        PathBuf::from(args.json_level_path.file_stem().unwrap()).with_extension("rs"),
    )?;

    // Now convert the levels as save as Rust code
    for rust_level in rust_levels {
        writeln!(rust_file, "{rust_level}")?;
    }

    Ok(())
}
