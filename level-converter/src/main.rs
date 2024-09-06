use clap::Parser;
use euclid::{Point2D, Size2D, Transform2D};
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::PathBuf};

// It would be nice to be able to use a doc string for the about but evidently there is no way
// to do this and have multiple lines:
// https://github.com/clap-rs/clap/issues/5003
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Converts Kuboble level definitions from JSON to Rust code and prints some metrics.\n\nThe JSON file must be in the format defined here: https://github.com/redking00/kubobleSolverJS"
)]
struct Args {
    /// JSON file to convert.
    json_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct JsonLevel {
    x: u8,
    y: u8,
    n: usize,
    s: Vec<u8>,
    t: Vec<u8>,
    o: Vec<u8>,
}
impl JsonLevel {
    pub fn size(&self) -> Size<JsonLevel> {
        Size::new(self.x, self.y)
    }
}

type Size<U> = Size2D<u8, U>;
type Point<U> = Point2D<u8, U>;

struct RustLevel {
    level_num: u16,
    spaces: Vec<String>,
    positions: Vec<Point<Self>>,
    optimal: u8,
}
impl From<JsonLevel> for RustLevel {
    fn from(value: JsonLevel) -> Self {
        todo!()
    }
}

trait SizeExt {
    fn into_rust(self) -> Size<RustLevel>;
}
impl SizeExt for Size<JsonLevel> {
    fn into_rust(self) -> Size<RustLevel> {
        Size::new(self.width + 2, self.height + 2)
    }
}

/*
// Level 13
    level! {
        spaces: &[
            "_#####",
            "_# G #",
            "###  #",
            "# O  #",
            "######",
        ],
        positions: [(3, 1), (4, 1)],
        optimal: 8,
    },
 */
fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Parse the JSON file
    let json_levels: Vec<JsonLevel> =
        serde_json::from_reader(BufReader::new(File::open(args.json_path)?))?;

    for (idx, json_level) in json_levels
        .into_iter()
        .take_while(|jl| jl.n < 3)
        .enumerate()
    {
        let json_size = json_level.size();
        println!("{}: {:?} {:?}", idx + 1, json_size, json_size.into_rust());
    }

    Ok(())
}
