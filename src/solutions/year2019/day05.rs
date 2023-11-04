use crate::intcode::{self, State};

pub fn part1(input: &str) -> i32 {
    let mut vm = intcode::parse(input);
    vm.input(1);
    loop {
        vm.run();
        let output = vm.output();
        if output != 0 {
            assert_eq!(vm.run(), State::Halt);
            return output;
        }
    }
}

pub fn part2(input: &str) -> i32 {
    let mut vm = intcode::parse(input);
    vm.input(5);
    vm.run();
    let output = vm.output();
    assert_eq!(vm.run(), State::Halt);
    output
}
