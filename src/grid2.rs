use lazy_static::lazy_static;
use nalgebra::Vector2;
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

pub trait IntoVector2 {
    fn into_vector2(self) -> Vector2<i32>;
}

impl IntoVector2 for char {
    fn into_vector2(self) -> Vector2<i32> {
        match self {
            'N' | 'U' | '^' => N,
            'E' | 'R' | '>' => E,
            'S' | 'D' | 'v' => S,
            'W' | 'L' | '<' => W,
            _ => panic!("don't know how to convert {self} in to a Vector2"),
        }
    }
}

impl IntoVector2 for &str {
    fn into_vector2(self) -> Vector2<i32> {
        lazy_static! {
            static ref INT: Regex = Regex::new(r"-?\d+").unwrap();
        }
        if self.len() == 1 {
            return self.chars().next().unwrap().into_vector2();
        }
        Vector2::from_iterator(INT.find_iter(self).map(|m| m.as_str().parse().unwrap()))
    }
}
