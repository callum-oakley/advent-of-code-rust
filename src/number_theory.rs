use std::cmp;

use num::{PrimInt, traits::Euclid};

/// Congruence of the form x = a (mod n)
pub struct Congruence<N> {
    pub a: N,
    pub n: N,
}

/// Given a system of congruences, finds x s.t. x = a (mod n) for every a and n.
pub fn chinese_remainder<N: PrimInt + Euclid>(mut system: Vec<Congruence<N>>) -> N {
    system.sort_unstable_by_key(|c| cmp::Reverse(c.n));

    for congruence in &mut system {
        congruence.a = congruence.a.rem_euclid(&congruence.n);
    }

    let mut x = system[0].a;
    let mut step = system[0].n;
    for &Congruence { a, n } in &system[1..] {
        while x % n != a {
            x = x + step;
        }
        step = step * n;
    }
    x
}

/// Wraps `n` to the range `low..high`
pub fn wrap<N: PrimInt + Euclid>(n: N, low: N, high: N) -> N {
    (n - low).rem_euclid(&(high - low)) + low
}
