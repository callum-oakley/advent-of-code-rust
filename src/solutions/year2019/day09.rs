use crate::intcode::VM;

pub fn part1(input: &str) -> i64 {
    let mut vm = VM::new(input);
    vm.input(1);
    let res = vm.output();
    vm.halt();
    res
}

pub fn part2(input: &str) -> i64 {
    let mut vm = VM::new(input);
    vm.input(2);
    let res = vm.output();
    vm.halt();
    res
}
