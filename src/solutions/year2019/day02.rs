use crate::intcode::{self, Memory, VM};

fn run(mut vm: VM, noun: usize, verb: usize) -> usize {
    vm.mem[1] = noun;
    vm.mem[2] = verb;
    vm.run();
    vm.mem[0]
}

pub fn part1(input: &str) -> usize {
    run(intcode::parse(input), 12, 2)
}

pub fn part2(input: &str) -> usize {
    let vm = intcode::parse(input);
    for noun in 0..100 {
        for verb in 0..100 {
            if run(vm.clone(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

pub fn tests() {
    fn part1_(input: &str) -> Memory {
        let mut vm = intcode::parse(input);
        vm.run();
        vm.mem
    }

    assert_eq!(
        &part1_("1,9,10,3,2,3,11,0,99,30,40,50")[..12],
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );
    assert_eq!(&part1_("1,0,0,0,99")[..5], &[2, 0, 0, 0, 99]);
    assert_eq!(&part1_("2,3,0,3,99")[..5], &[2, 3, 0, 6, 99]);
    assert_eq!(&part1_("2,4,4,5,99,0")[..6], &[2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        &part1_("1,1,1,4,99,5,6,0,99")[..9],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}
