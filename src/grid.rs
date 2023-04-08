use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub const Z: Point = Point { x: 0, y: 0 };
pub const N: Point = Point { x: 0, y: -1 };
pub const E: Point = Point { x: 1, y: 0 };
pub const S: Point = Point { x: 0, y: 1 };
pub const W: Point = Point { x: -1, y: 0 };

impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            '^' => N,
            '>' => E,
            'v' => S,
            '<' => W,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref INT: Regex = Regex::new(r"-?\d+").unwrap();
        }
        let mut ints = INT.find_iter(s);
        Point {
            x: ints.next().unwrap().as_str().parse().unwrap(),
            y: ints.next().unwrap().as_str().parse().unwrap(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, scalar: i32) {
        *self = *self * scalar;
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, scalar: i32) {
        *self = *self / scalar;
    }
}
