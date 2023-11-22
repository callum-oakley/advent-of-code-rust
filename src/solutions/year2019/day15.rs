use crate::{
    grid::{Point, E, N, S, W, Z},
    intcode::VM,
    search2::{self, Queue},
};

#[derive(Clone)]
struct State {
    vm: VM,
    pos: Point,
    steps: u32,
    found_oxygen_system: bool,
}

impl State {
    fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        [N, S, W, E].iter().enumerate().map(|(i, dir)| {
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
        })
    }
}

fn oxygen_system(input: &str) -> State {
    let mut q = search2::breadth_first(
        State {
            vm: VM::new(input),
            pos: Z,
            steps: 0,
            found_oxygen_system: false,
        },
        |state| state.pos,
    );
    while let Some(state) = q.pop() {
        if state.found_oxygen_system {
            return state;
        }
        for s in state.adjacent() {
            q.push(s);
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> u32 {
    oxygen_system(input).steps
}

pub fn part2(input: &str) -> u32 {
    let mut q = search2::breadth_first(
        State {
            steps: 0,
            ..oxygen_system(input)
        },
        |state| state.pos,
    );
    let mut max_steps = 0;
    while let Some(state) = q.pop() {
        max_steps = max_steps.max(state.steps);
        for s in state.adjacent() {
            q.push(s);
        }
    }
    max_steps
}
