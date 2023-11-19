use std::{
    fmt::{self, Write},
    iter::{self, Sum},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub y: i32,
    pub x: i32,
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

#[derive(Copy, Clone, PartialEq, Debug)]
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

impl From<i64> for Turn {
    fn from(s: i64) -> Self {
        match s {
            0 => Turn::Left,
            1 => Turn::Right,
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
        if s.len() == 1 {
            return s.chars().next().unwrap().into();
        }
        let mut ints = INT.find_iter(s);
        Point {
            x: ints.next().unwrap().as_str().parse().unwrap(),
            y: ints.next().unwrap().as_str().parse().unwrap(),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
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

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
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

impl Div for Point {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
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

impl Rem for Point {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
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

impl MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, scalar: i32) {
        *self = *self * scalar;
    }
}

impl DivAssign for Point {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, scalar: i32) {
        *self = *self / scalar;
    }
}

impl RemAssign for Point {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

impl Sum for Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Z, |a, b| a + b)
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub min_y: i32,
    pub max_y: i32,
    pub min_x: i32,
    pub max_x: i32,
}

impl Bounds {
    pub fn new(mut points: impl Iterator<Item = Point>) -> Self {
        let point = points.next().unwrap();
        let mut res = Self {
            min_y: point.y,
            max_y: point.y,
            min_x: point.x,
            max_x: point.x,
        };

        for point in points {
            res.min_y = res.min_y.min(point.y);
            res.max_y = res.max_y.max(point.y);
            res.min_x = res.min_x.min(point.x);
            res.max_x = res.max_x.max(point.x);
        }

        res
    }

    pub fn size(&self) -> Point {
        Point {
            y: self.max_y - self.min_y + 1,
            x: self.max_x - self.min_x + 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

pub fn scan_rect<F: FnMut(Point, char)>(s: &str, mut f: F) {
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            f(
                Point {
                    y: y.try_into().unwrap(),
                    x: x.try_into().unwrap(),
                },
                c,
            );
        }
    }
}

impl<T> Rect<T> {
    pub fn parse<F: FnMut(Point, char) -> T>(s: &str, mut f: F) -> Self {
        let mut inner = Vec::new();
        scan_rect(s, |pos, c| {
            while inner.len() <= usize::try_from(pos.y).unwrap() {
                inner.push(Vec::new());
            }
            inner[usize::try_from(pos.y).unwrap()].push(f(pos, c));
        });
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

    pub fn keys(&self) -> impl Iterator<Item = Point> {
        let size = self.size;
        let mut pos = Z;
        iter::from_fn(move || {
            if pos.y >= size.y {
                None
            } else {
                let res = pos;
                pos.x += 1;
                if pos.x >= size.x {
                    pos.x = 0;
                    pos.y += 1;
                }
                Some(res)
            }
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.keys().map(|pos| &self[pos])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.keys().map(|pos| (pos, &self[pos]))
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

impl<T> Index<&Point> for Rect<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &T {
        &self.inner[usize::try_from(index.y).unwrap()][usize::try_from(index.x).unwrap()]
    }
}

impl<T> IndexMut<&Point> for Rect<T> {
    fn index_mut(&mut self, index: &Point) -> &mut T {
        &mut self.inner[usize::try_from(index.y).unwrap()][usize::try_from(index.x).unwrap()]
    }
}

impl<I> From<I> for Rect<bool>
where
    I: Iterator<Item = Point> + Clone,
{
    fn from(points: I) -> Self {
        let bounds = Bounds::new(points.clone());
        let mut res = Self::new(false, bounds.size());
        for point in points {
            res[point
                - Point {
                    y: bounds.min_y,
                    x: bounds.min_x,
                }] = true;
        }
        res
    }
}

impl fmt::Display for Rect<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                f.write_char(if self[Point { y, x }] { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<'a, T> IntoIterator for &'a Rect<T> {
    type Item = (Point, &'a T);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}
