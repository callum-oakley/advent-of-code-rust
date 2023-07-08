use std::{
    fmt::{self, Write},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub const NW: Point = Point { x: -1, y: -1 };
pub const N: Point = Point { x: 0, y: -1 };
pub const NE: Point = Point { x: 1, y: -1 };
pub const W: Point = Point { x: -1, y: 0 };
pub const Z: Point = Point { x: 0, y: 0 };
pub const E: Point = Point { x: 1, y: 0 };
pub const SW: Point = Point { x: -1, y: 1 };
pub const S: Point = Point { x: 0, y: 1 };
pub const SE: Point = Point { x: 1, y: 1 };

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
    pub fn adjacent4(self) -> [Self; 4] {
        [N, W, E, S].map(|dir| dir + self)
    }

    pub fn adjacent8(self) -> [Self; 8] {
        [NW, N, NE, W, E, SW, S, SE].map(|dir| dir + self)
    }

    pub fn manhattan(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn turn(self, t: Turn) -> Self {
        match t {
            Turn::Left => Point {
                x: self.y,
                y: -self.x,
            },
            Turn::Right => Point {
                x: -self.y,
                y: self.x,
            },
        }
    }
}

impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            'N' | 'U' | '^' => N,
            'E' | 'R' | '>' => E,
            'S' | 'D' | 'v' => S,
            'W' | 'L' | '<' => W,
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

impl Sum for Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Z, |a, b| a + b)
    }
}

pub struct Rect<T> {
    inner: Vec<Vec<T>>,
    pub size: Point,
}

impl<T: Clone> Rect<T> {
    pub fn new(init: T, size: Point) -> Self {
        Self {
            inner: vec![
                vec![init; usize::try_from(size.x).unwrap()];
                usize::try_from(size.y).unwrap()
            ],
            size,
        }
    }
}

impl<T> Rect<T> {
    pub fn parse<F: Fn(char) -> T>(s: &str, f: F) -> Self {
        let mut inner = Vec::new();
        for line in s.lines() {
            inner.push(Vec::new());
            for c in line.chars() {
                inner.last_mut().unwrap().push(f(c));
            }
        }
        let size = if inner.is_empty() {
            Point { x: 0, y: 0 }
        } else {
            Point {
                x: i32::try_from(inner[0].len()).unwrap(),
                y: i32::try_from(inner.len()).unwrap(),
            }
        };
        Self { inner, size }
    }

    pub fn get(&self, index: Point) -> Option<&T> {
        if index.y < 0 || index.x < 0 {
            None
        } else {
            self.inner
                .get(usize::try_from(index.y).unwrap())
                .and_then(|row| row.get(usize::try_from(index.x).unwrap()))
        }
    }

    pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
        if index.y < 0 || index.x < 0 {
            None
        } else {
            self.inner
                .get_mut(usize::try_from(index.y).unwrap())
                .and_then(|row| row.get_mut(usize::try_from(index.x).unwrap()))
        }
    }

    pub fn iter(&self) -> RectIter<T> {
        RectIter {
            grid: self,
            index: Z,
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = Point> + '_ {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.iter().map(|(_, value)| value)
    }
}

impl<T> Index<Point> for Rect<T> {
    type Output = T;

    fn index(&self, index: Point) -> &T {
        &self.inner[usize::try_from(index.y).unwrap()][usize::try_from(index.x).unwrap()]
    }
}

impl<T> IndexMut<Point> for Rect<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        &mut self.inner[usize::try_from(index.y).unwrap()][usize::try_from(index.x).unwrap()]
    }
}

impl fmt::Display for Rect<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                f.write_char(if self[Point { x, y }] { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<'a, T> IntoIterator for &'a Rect<T> {
    type Item = (Point, &'a T);
    type IntoIter = RectIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct RectIter<'a, T> {
    grid: &'a Rect<T>,
    index: Point,
}

impl<'a, T> Iterator for RectIter<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.get(self.index).map(|value| {
            let index = self.index;
            self.index.x += 1;
            if self.index.x >= self.grid.size.x {
                self.index.x = 0;
                self.index.y += 1;
            }
            (index, value)
        })
    }
}
