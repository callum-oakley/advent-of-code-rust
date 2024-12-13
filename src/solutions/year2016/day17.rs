use md5::{Digest, Md5};

use crate::{
    grid::{IntoVector, Vector, Z},
    search2,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    path: String,
    pos: Vector,
}

fn adjacent(input: &str, state: &State, push: &mut dyn FnMut(State)) {
    if state.pos == Vector::new(3, 3) {
        return;
    }

    let mut hasher = Md5::new();
    hasher.update(input);
    hasher.update(&state.path);
    let hash = hasher.finalize();

    if state.pos.y > 0 && hash[0] >> 4 > 10 {
        let mut state = state.clone();
        state.path.push('U');
        state.pos += 'U'.into_vector();
        push(state);
    }
    if state.pos.y < 3 && hash[0] & 0xf > 10 {
        let mut state = state.clone();
        state.path.push('D');
        state.pos += 'D'.into_vector();
        push(state);
    }
    if state.pos.x > 0 && hash[1] >> 4 > 10 {
        let mut state = state.clone();
        state.path.push('L');
        state.pos += 'L'.into_vector();
        push(state);
    }
    if state.pos.x < 3 && hash[1] & 0xf > 10 {
        let mut state = state.clone();
        state.path.push('R');
        state.pos += 'R'.into_vector();
        push(state);
    }
}

pub fn part1(input: &str) -> String {
    search2::breadth_first(
        State {
            path: String::new(),
            pos: Z,
        },
        Clone::clone,
        |state, push| adjacent(input, state, push),
    )
    .find(|state| state.pos == Vector::new(3, 3))
    .unwrap()
    .path
}

pub fn part2(input: &str) -> usize {
    search2::breadth_first(
        State {
            path: String::new(),
            pos: Z,
        },
        Clone::clone,
        |state, push| adjacent(input, state, push),
    )
    .filter(|state| state.pos == Vector::new(3, 3))
    .map(|state| state.path.len())
    .max()
    .unwrap()
}

pub fn tests() {
    assert_eq!(part1("ihgpwlah"), "DDRRRD");
    assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
    assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");

    assert_eq!(part2("ihgpwlah"), 370);
    assert_eq!(part2("kglvqrro"), 492);
    assert_eq!(part2("ulqzkmiv"), 830);
}
