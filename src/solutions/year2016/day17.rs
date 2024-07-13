use md5::{Digest, Md5};

use crate::{
    grid::{IntoVector, Vector, Z},
    search::{self, Queue},
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    path: String,
    pos: Vector,
}

impl State {
    fn push_adjacent(&self, input: &str, q: &mut impl Queue<Item = Self>) {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(&self.path);
        let hash = hasher.finalize();

        if self.pos.y > 0 && hash[0] >> 4 > 10 {
            let mut state = self.clone();
            state.path.push('U');
            state.pos += 'U'.into_vector();
            q.push(state);
        }
        if self.pos.y < 3 && hash[0] & 0xf > 10 {
            let mut state = self.clone();
            state.path.push('D');
            state.pos += 'D'.into_vector();
            q.push(state);
        }
        if self.pos.x > 0 && hash[1] >> 4 > 10 {
            let mut state = self.clone();
            state.path.push('L');
            state.pos += 'L'.into_vector();
            q.push(state);
        }
        if self.pos.x < 3 && hash[1] & 0xf > 10 {
            let mut state = self.clone();
            state.path.push('R');
            state.pos += 'R'.into_vector();
            q.push(state);
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut q = search::breadth_first(
        State {
            path: String::new(),
            pos: Z,
        },
        Clone::clone,
    );
    while let Some(state) = q.pop() {
        if state.pos == Vector::new(3, 3) {
            return state.path;
        }
        state.push_adjacent(input, &mut q);
    }
    unreachable!()
}

pub fn part2(input: &str) -> usize {
    let mut q = search::breadth_first(
        State {
            path: String::new(),
            pos: Z,
        },
        Clone::clone,
    );
    let mut res = 0;
    while let Some(state) = q.pop() {
        if state.pos == Vector::new(3, 3) {
            res = res.max(state.path.len());
        } else {
            state.push_adjacent(input, &mut q);
        }
    }
    res
}

pub fn tests() {
    assert_eq!(part1("ihgpwlah"), "DDRRRD");
    assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
    assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");

    assert_eq!(part2("ihgpwlah"), 370);
    assert_eq!(part2("kglvqrro"), 492);
    assert_eq!(part2("ulqzkmiv"), 830);
}
