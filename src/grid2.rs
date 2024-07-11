use lazy_static::lazy_static;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

pub const NW: Vector2<i32> = Vector2::new(-1, -1);
pub const N: Vector2<i32> = Vector2::new(0, -1);
pub const NE: Vector2<i32> = Vector2::new(1, -1);
pub const W: Vector2<i32> = Vector2::new(-1, 0);
pub const Z: Vector2<i32> = Vector2::new(0, 0);
pub const E: Vector2<i32> = Vector2::new(1, 0);
pub const SW: Vector2<i32> = Vector2::new(-1, 1);
pub const S: Vector2<i32> = Vector2::new(0, 1);
pub const SE: Vector2<i32> = Vector2::new(1, 1);

pub const LEFT: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
pub const RIGHT: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);

pub trait IntoVector {
    fn into_vector(self) -> Vector2<i32>;
}

impl IntoVector for char {
    fn into_vector(self) -> Vector2<i32> {
        match self {
            'N' | 'U' | '^' => N,
            'E' | 'R' | '>' => E,
            'S' | 'D' | 'v' => S,
            'W' | 'L' | '<' => W,
            _ => panic!("don't know how to convert {self} into a vector"),
        }
    }
}

impl IntoVector for &str {
    fn into_vector(self) -> Vector2<i32> {
        if self.len() == 1 {
            return self.chars().next().unwrap().into_vector();
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(-?\d+)[^-\d]+(-?\d+)").unwrap();
        }
        let Some(c) = RE.captures(self) else {
            panic!("don't know how to convert {self} into a vector");
        };
        Vector2::new(c[1].parse().unwrap(), c[2].parse().unwrap())
    }
}

pub trait IntoTurn {
    fn into_turn(self) -> Matrix2<i32>;
}

impl IntoTurn for char {
    fn into_turn(self) -> Matrix2<i32> {
        match self {
            'L' => LEFT,
            'R' => RIGHT,
            _ => panic!("don't know how to convert {self} into a turn"),
        }
    }
}

impl IntoTurn for &str {
    fn into_turn(self) -> Matrix2<i32> {
        match self {
            "L" => LEFT,
            "R" => RIGHT,
            _ => panic!("don't know how to convert {self} into a turn"),
        }
    }
}

impl IntoTurn for i64 {
    fn into_turn(self) -> Matrix2<i32> {
        match self {
            0 => LEFT,
            1 => RIGHT,
            _ => panic!("don't know how to convert {self} into a turn"),
        }
    }
}
