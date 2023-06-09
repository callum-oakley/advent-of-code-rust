use std::collections::HashMap;

use crate::grid_3d::Point;

struct Particle {
    p: Point,
    v: Point,
    a: Point,
}

impl Particle {
    fn tick(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

impl From<&str> for Particle {
    fn from(s: &str) -> Self {
        let mut points = s.split(", ");
        Particle {
            p: points.next().unwrap().into(),
            v: points.next().unwrap().into(),
            a: points.next().unwrap().into(),
        }
    }
}

fn parse(input: &str) -> HashMap<usize, Particle> {
    input.lines().map(Particle::from).enumerate().collect()
}

fn collide(system: &mut HashMap<usize, Particle>) {
    let mut seen = HashMap::new();
    for i in system.keys().copied().collect::<Vec<_>>() {
        let p = system.get_mut(&i).unwrap();
        p.tick();
        if let Some(&j) = seen.get(&p.p) {
            system.remove(&i);
            system.remove(&j);
        } else {
            seen.insert(p.p, i);
        }
    }
}

// The particle with the smallest acceleration will ultimately be the one to
// stay closest to Z. Ties broken by initial velocity, then initial position.
pub fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .min_by_key(|(_, p)| (p.a.manhattan(), p.v.manhattan(), p.p.manhattan()))
        .unwrap()
        .0
}

pub fn part2(input: &str) -> usize {
    let mut system = parse(input);
    for _ in 0..100 {
        collide(&mut system);
    }
    system.len()
}

pub fn tests() {
    assert_eq!(
        part1(
            "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
             p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>",
        ),
        0,
    );
    assert_eq!(
        part2(
            "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
             p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
             p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
             p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>",
        ),
        1,
    );
}
