use std::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
    str::FromStr,
};

use lazy_static::lazy_static;
use nalgebra::{SVector, Scalar};
use regex::Regex;

pub type Vector = nalgebra::Vector2<i32>;
pub type Turn = nalgebra::Matrix2<i32>;

pub const NW: Vector = Vector::new(-1, -1);
pub const N: Vector = Vector::new(0, -1);
pub const NE: Vector = Vector::new(1, -1);
pub const W: Vector = Vector::new(-1, 0);
pub const Z: Vector = Vector::new(0, 0);
pub const E: Vector = Vector::new(1, 0);
pub const SW: Vector = Vector::new(-1, 1);
pub const S: Vector = Vector::new(0, 1);
pub const SE: Vector = Vector::new(1, 1);

pub const LEFT: Turn = Turn::new(0, 1, -1, 0);
pub const RIGHT: Turn = Turn::new(0, -1, 1, 0);

pub fn adjacent4<V: Into<Vector>>(v: V) -> impl Iterator<Item = Vector> {
    let v = v.into();
    [N, W, E, S].into_iter().map(move |dir| dir + v)
}

pub fn adjacent8<V: Into<Vector>>(v: V) -> impl Iterator<Item = Vector> {
    let v = v.into();
    [NW, N, NE, W, E, SW, S, SE]
        .into_iter()
        .map(move |dir| dir + v)
}

pub fn reading_ord_key(v: Vector) -> [i32; 2] {
    [v.y, v.x]
}

pub trait IntoVector<T, const D: usize> {
    fn into_vector(self) -> SVector<T, D>;
}

impl IntoVector<i32, 2> for char {
    fn into_vector(self) -> Vector {
        match self {
            'N' | 'U' | '^' => N,
            'E' | 'R' | '>' => E,
            'S' | 'D' | 'v' => S,
            'W' | 'L' | '<' => W,
            _ => panic!("don't know how to convert {self} into a vector"),
        }
    }
}

impl<T, const D: usize> IntoVector<T, D> for &str
where
    T: Scalar + FromStr,
    T::Err: fmt::Debug,
{
    fn into_vector(self) -> SVector<T, D> {
        lazy_static! {
            static ref INTS: Regex = Regex::new(r"-?\d+").unwrap();
        }
        SVector::from_iterator(INTS.find_iter(self).map(|m| m.as_str().parse().unwrap()))
    }
}

pub trait IntoTurn {
    fn into_turn(self) -> Turn;
}

impl IntoTurn for char {
    fn into_turn(self) -> Turn {
        match self {
            'L' => LEFT,
            'R' => RIGHT,
            _ => panic!("don't know how to convert {self} into a turn"),
        }
    }
}

impl IntoTurn for &str {
    fn into_turn(self) -> Turn {
        self.chars().next().unwrap().into_turn()
    }
}

pub trait IntoChar {
    fn into_char(self) -> char;
}

impl IntoChar for Vector {
    fn into_char(self) -> char {
        if self == N {
            'N'
        } else if self == W {
            'W'
        } else if self == Z {
            'Z'
        } else if self == E {
            'E'
        } else if self == S {
            'S'
        } else {
            panic!("don't know how to convert {self} into a char")
        }
    }
}

impl IntoChar for Turn {
    fn into_char(self) -> char {
        if self == LEFT {
            'L'
        } else if self == RIGHT {
            'R'
        } else {
            panic!("don't know how to convert {self} into a char")
        }
    }
}

pub fn scan<F: FnMut(Vector, char)>(s: &str, mut f: F) -> Vector {
    let mut size = Z;
    let mut width = None;
    for c in s.chars() {
        if c == '\n' {
            if let Some(w) = width {
                assert!(w == size.x, "string is not rectangular");
            } else {
                width = Some(size.x);
            }
            size.x = 0;
            size.y += 1;
        } else {
            f(size, c);
            size.x += 1;
        }
    }
    size.y += 1;
    size
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    pub size: Vector,
}

impl<T> Grid<T> {
    pub fn new<V: Into<Vector>>(init: T, size: V) -> Self
    where
        T: Clone,
    {
        let size: Vector = size.into();
        let data = vec![init; usize::try_from(size.x * size.y).unwrap()];
        Self { data, size }
    }

    pub fn parse<F: FnMut(Vector, char) -> T>(s: &str, mut f: F) -> Self {
        let mut data = Vec::new();
        let size = scan(s, |v, c| data.push(f(v, c)));
        Self { data, size }
    }

    pub fn get<V: Into<Vector>>(&self, v: V) -> Option<&T> {
        let v: Vector = v.into();
        if 0 <= v.x && v.x < self.size.x && 0 <= v.y && v.y < self.size.y {
            Some(&self.data[usize::try_from(v.x + self.size.x * v.y).unwrap()])
        } else {
            None
        }
    }

    pub fn get_mut<V: Into<Vector>>(&mut self, v: V) -> Option<&mut T> {
        let v: Vector = v.into();
        if 0 <= v.x && v.x < self.size.x && 0 <= v.y && v.y < self.size.y {
            Some(&mut self.data[usize::try_from(v.x + self.size.x * v.y).unwrap()])
        } else {
            None
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = Vector> {
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
        self.data.iter()
    }

    pub fn into_values(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vector, &T)> {
        self.keys().map(|k| (k, &self[k]))
    }

    pub fn adjacent4(&self, v: Vector) -> impl Iterator<Item = (Vector, &T)> {
        adjacent4(v).filter_map(|u| self.get(u).map(|t| (u, t)))
    }

    pub fn adjacent8(&self, v: Vector) -> impl Iterator<Item = (Vector, &T)> {
        adjacent8(v).filter_map(|u| self.get(u).map(|t| (u, t)))
    }

    pub fn adjacent4_values(&self, v: Vector) -> impl Iterator<Item = &T> {
        adjacent4(v).filter_map(|u| self.get(u))
    }

    pub fn adjacent8_values(&self, v: Vector) -> impl Iterator<Item = &T> {
        adjacent8(v).filter_map(|u| self.get(u))
    }

    pub fn fmt_with<F: FnMut(&T) -> char>(
        &self,
        f: &mut fmt::Formatter<'_>,
        mut to_char: F,
    ) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                f.write_char(to_char(&self[[x, y]]))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T, V> Index<V> for Grid<T>
where
    V: Into<Vector>,
{
    type Output = T;

    fn index(&self, v: V) -> &Self::Output {
        self.get(v).unwrap()
    }
}

impl<T, V> IndexMut<V> for Grid<T>
where
    V: Into<Vector>,
{
    fn index_mut(&mut self, v: V) -> &mut Self::Output {
        self.get_mut(v).unwrap()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Vector, &'a T);

    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, |t| if *t { '#' } else { '.' })
    }
}

impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, |t| *t)
    }
}

pub struct Bounds<const D: usize> {
    pub min: SVector<i32, D>,
    pub max: SVector<i32, D>,
}

impl<const D: usize> Bounds<D> {
    pub fn new(mut points: impl Iterator<Item = SVector<i32, D>>) -> Self {
        let point = points.next().unwrap();
        let mut res = Self {
            min: point,
            max: point,
        };

        for point in points {
            for (axis, &element) in point.iter().enumerate() {
                res.min[axis] = res.min[axis].min(element);
                res.max[axis] = res.max[axis].max(element);
            }
        }

        res
    }

    pub fn size(&self) -> SVector<i32, D> {
        self.max - self.min + SVector::from_element(1)
    }
}

impl<I> From<I> for Grid<bool>
where
    I: IntoIterator<Item = Vector> + Clone,
{
    fn from(points: I) -> Self {
        let bounds = Bounds::new(points.clone().into_iter());
        let mut res = Self::new(false, bounds.size());
        for point in points {
            res[point - bounds.min] = true;
        }
        res
    }
}
