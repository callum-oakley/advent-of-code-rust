use crate::intcode::VM;

pub fn part1(input: &str) -> i32 {
    let mut vm = VM::new(input);
    vm.input(1);
    loop {
        let output = vm.output();
        if output != 0 {
            vm.halt();
            return output;
        }
    }
}

pub fn part2(input: &str) -> i32 {
    let mut vm = VM::new(input);
    vm.input(5);
    let output = vm.output();
    vm.halt();
    output
}
