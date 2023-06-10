use std::cmp;

/// Congruence of the form x = a (mod n)
pub struct Congruence {
    pub a: i64,
    pub n: i64,
}

/// Given a system of congruences, finds x s.t. x = a (mod n) for every a and n.
pub fn solve(mut system: Vec<Congruence>) -> i64 {
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
