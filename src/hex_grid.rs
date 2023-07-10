pub use crate::grid::{Point, Z};

// As described here (in the "flat" orientation):
// <https://www.redblobgames.com/grids/hexagons/#coordinates-axial>
// Let x correspond to q and y correspond to r:
pub const N: Point = Point { x: 0, y: -1 };
pub const NE: Point = Point { x: 1, y: -1 };
pub const SE: Point = Point { x: 1, y: 0 };
pub const S: Point = Point { x: 0, y: 1 };
pub const SW: Point = Point { x: -1, y: 1 };
pub const NW: Point = Point { x: -1, y: 0 };

pub fn dist(p: Point) -> i32 {
    (p.x.abs() + p.y.abs() + (p.x + p.y).abs()) / 2
}

pub fn from_str(s: &str) -> Point {
    match s {
        "n" => N,
        "ne" => NE,
        "se" => SE,
        "s" => S,
        "sw" => SW,
        "nw" => NW,
        _ => unreachable!(),
    }
}
