use crate::intcode::{State, VM};

pub fn play<F>(input: &str, mut handle_output: F) -> i64
where
    F: FnMut(i64, i64, i64),
{
    let mut vm = VM::new(input);
    vm.mem[0] = 2;

    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        match vm.state() {
            State::Input => {
                vm.input(match paddle_x.cmp(&ball_x) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => -1,
                });
            }
            State::Output => {
                let x = vm.output();
                let y = vm.output();
                let z = vm.output();

                if x == -1 && y == 0 {
                    score = z;
                } else if z == 3 {
                    paddle_x = x;
                } else if z == 4 {
                    ball_x = x;
                }

                handle_output(x, y, z);
            }
            State::Halt => {
                return score;
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut vm = VM::new(input);
    let mut res = 0;
    while vm.state() != State::Halt {
        vm.output();
        vm.output();
        if vm.output() == 2 {
            res += 1;
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    play(input, |_, _, _| {})
}
