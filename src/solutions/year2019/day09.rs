use std::fmt::Write;

use crate::intcode::{State, VM};

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

pub fn tests() {
    let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut vm = VM::new(quine);
    let mut output = vm.output().to_string();
    while vm.state() == State::Output {
        write!(output, ",{}", vm.output()).unwrap();
    }
    assert_eq!(output, quine);
    vm.halt();

    let mut vm = VM::new("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(vm.output(), 1_219_070_632_396_864);
    vm.halt();

    let mut vm = VM::new("104,1125899906842624,99");
    assert_eq!(vm.output(), 1_125_899_906_842_624);
    vm.halt();
}
