use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// As described here (in the "flat" orientation):
/// <https://www.redblobgames.com/grids/hexagons/#coordinates-axial>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Axial {
    q: i32,
    r: i32,
}

pub const Z: Axial = Axial { q: 0, r: 0 };
pub const N: Axial = Axial { q: 0, r: -1 };
pub const NE: Axial = Axial { q: 1, r: -1 };
pub const SE: Axial = Axial { q: 1, r: 0 };
pub const S: Axial = Axial { q: 0, r: 1 };
pub const SW: Axial = Axial { q: -1, r: 1 };
pub const NW: Axial = Axial { q: -1, r: 0 };

impl Axial {
    pub fn dist(self) -> i32 {
        (self.q.abs() + self.r.abs() + (self.q + self.r).abs()) / 2
    }
}

impl From<&str> for Axial {
    fn from(s: &str) -> Self {
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
}

impl Add for Axial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl Sub for Axial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

impl Mul<i32> for Axial {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            q: self.q * scalar,
            r: self.r * scalar,
        }
    }
}

impl Div<i32> for Axial {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        Self {
            q: self.q / scalar,
            r: self.r / scalar,
        }
    }
}

impl AddAssign for Axial {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Axial {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign<i32> for Axial {
    fn mul_assign(&mut self, scalar: i32) {
        *self = *self * scalar;
    }
}

impl DivAssign<i32> for Axial {
    fn div_assign(&mut self, scalar: i32) {
        *self = *self / scalar;
    }
}

impl Sum for Axial {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Z, |a, b| a + b)
    }
}
