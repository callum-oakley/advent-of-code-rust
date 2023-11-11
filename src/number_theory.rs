use std::{
    cmp,
    ops::{Div, Mul, Rem},
};

/// Congruence of the form x = a (mod n)
pub struct Congruence {
    pub a: i64,
    pub n: i64,
}

/// Given a system of congruences, finds x s.t. x = a (mod n) for every a and n.
pub fn chinese_remainder(mut system: Vec<Congruence>) -> i64 {
    system.sort_unstable_by_key(|c| cmp::Reverse(c.n));

    for congruence in &mut system {
        congruence.a = congruence.a.rem_euclid(congruence.n);
    }

    let mut x = system[0].a;
    let mut step = system[0].n;
    for &Congruence { a, n } in &system[1..] {
        while x % n != a {
            x += step;
        }
        step *= n;
    }
    x
}

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq + Copy + Default + Rem<Output = T>,
{
    // Assume T::default() is 0. Definitely an abuse of the trait but true for all the types we care
    // about.
    while b != T::default() {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq + Copy + Default + Rem<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    a * (b / gcd(a, b))
}
