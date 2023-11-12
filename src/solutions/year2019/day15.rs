use crate::{
    grid::{Point, E, N, S, W, Z},
    intcode::VM,
    search,
};

#[derive(Clone)]
struct State {
    vm: VM,
    pos: Point,
    steps: u32,
    found_oxygen_system: bool,
}

impl search::State for State {
    type HashKey = Point;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new([N, S, W, E].iter().enumerate().map(|(i, dir)| {
            let mut state = self.clone();
            state.vm.input(i64::try_from(i).unwrap() + 1);
            match state.vm.output() {
                0 => {}
                1 => {
                    state.pos += *dir;
                    state.steps += 1;
                }
                2 => {
                    state.pos += *dir;
                    state.steps += 1;
                    state.found_oxygen_system = true;
                }
                _ => unreachable!(),
            }
            state
        }))
    }

    fn hash_key(&self) -> Self::HashKey {
        self.pos
    }
}

fn oxygen_system(input: &str) -> State {
    search::breadth_first(State {
        vm: VM::new(input),
        pos: Z,
        steps: 0,
        found_oxygen_system: false,
    })
    .find(|state| state.found_oxygen_system)
    .unwrap()
}

pub fn part1(input: &str) -> u32 {
    oxygen_system(input).steps
}

pub fn part2(input: &str) -> u32 {
    let oxygen_system = oxygen_system(input);
    search::breadth_first(State {
        steps: 0,
        ..oxygen_system
    })
    .map(|state| state.steps)
    .max()
    .unwrap()
}
