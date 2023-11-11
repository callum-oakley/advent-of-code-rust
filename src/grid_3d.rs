use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
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

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Index<Axis> for Point {
    type Output = i32;

    fn index(&self, axis: Axis) -> &i32 {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl IndexMut<Axis> for Point {
    fn index_mut(&mut self, axis: Axis) -> &mut i32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub min_z: i32,
    pub max_z: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_x: i32,
    pub max_x: i32,
}

impl Bounds {
    pub fn new(mut points: impl Iterator<Item = Point>) -> Self {
        let point = points.next().unwrap();
        let mut res = Self {
            min_z: point.z,
            max_z: point.z,
            min_y: point.y,
            max_y: point.y,
            min_x: point.x,
            max_x: point.x,
        };

        for point in points {
            res.min_z = res.min_z.min(point.z);
            res.max_z = res.max_z.max(point.z);
            res.min_y = res.min_y.min(point.y);
            res.max_y = res.max_y.max(point.y);
            res.min_x = res.min_x.min(point.x);
            res.max_x = res.max_x.max(point.x);
        }

        res
    }
}
