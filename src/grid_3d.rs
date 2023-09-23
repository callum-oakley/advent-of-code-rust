use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub z: i32,
    pub y: i32,
    pub x: i32,
}

pub const Z: Point = Point { x: 0, y: 0, z: 0 };

#[derive(Copy, Clone, Debug)]
pub enum Turn {
    Left,
    Right,
}

impl From<&str> for Turn {
    fn from(s: &str) -> Self {
        match s {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => unreachable!(),
        }
    }
}

impl Point {
    pub fn manhattan(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
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
            z: ints.next().unwrap().as_str().parse().unwrap(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
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

impl Sum for Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Z, |a, b| a + b)
    }
}
