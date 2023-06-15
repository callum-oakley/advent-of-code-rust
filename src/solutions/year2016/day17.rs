use md5::{Digest, Md5};

use crate::{
    grid::{Point, Z},
    search,
};

#[derive(Clone)]
struct State<'input> {
    input: &'input str,
    path: String,
    pos: Point,
}

impl<'a, 'input> search::State for &'a State<'input> {
    type Adjacent = Vec<State<'input>>;
    type HashKey = &'a str;

    fn adjacent(self) -> Self::Adjacent {
        let mut res = Vec::new();

        if self.pos == (Point { x: 3, y: 3 }) {
            return res;
        }

        let mut hasher = Md5::new();
        hasher.update(self.input);
        hasher.update(&self.path);
        let hash = hasher.finalize();

        if self.pos.y > 0 && hash[0] >> 4 > 10 {
            let mut state = self.clone();
            state.path.push('U');
            state.pos += 'U'.into();
            res.push(state);
        }
        if self.pos.y < 3 && hash[0] & 0xf > 10 {
            let mut state = self.clone();
            state.path.push('D');
            state.pos += 'D'.into();
            res.push(state);
        }
        if self.pos.x > 0 && hash[1] >> 4 > 10 {
            let mut state = self.clone();
            state.path.push('L');
            state.pos += 'L'.into();
            res.push(state);
        }
        if self.pos.x < 3 && hash[1] & 0xf > 10 {
            let mut state = self.clone();
            state.path.push('R');
            state.pos += 'R'.into();
            res.push(state);
        }

        res
    }

    fn hash_key(self) -> Self::HashKey {
        &self.path
    }
}

pub fn part1(input: &str) -> String {
    search::breadth_first(State {
        input,
        path: String::new(),
        pos: Z,
    })
    .find(|state| state.pos == Point { x: 3, y: 3 })
    .unwrap()
    .path
}

pub fn part2(input: &str) -> usize {
    search::depth_first(State {
        input,
        path: String::new(),
        pos: Z,
    })
    .filter(|state| state.pos == Point { x: 3, y: 3 })
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
