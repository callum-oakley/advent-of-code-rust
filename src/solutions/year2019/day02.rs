use crate::intcode::{self, State, VM};

fn run(mut vm: VM, noun: i32, verb: i32) -> i32 {
    vm.mem[1] = noun;
    vm.mem[2] = verb;
    assert_eq!(vm.run(), State::Halt);
    vm.mem[0]
}

pub fn part1(input: &str) -> i32 {
    run(intcode::parse(input), 12, 2)
}

pub fn part2(input: &str) -> i32 {
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
    fn assert_mem(input: &str, expected: &[i32]) {
        let mut vm = intcode::parse(input);
        assert_eq!(vm.run(), State::Halt);
        assert_eq!(&vm.mem[..expected.len()], expected);
    }

    assert_mem(
        "1,9,10,3,2,3,11,0,99,30,40,50",
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );
    assert_mem("1,0,0,0,99", &[2, 0, 0, 0, 99]);
    assert_mem("2,3,0,3,99", &[2, 3, 0, 6, 99]);
    assert_mem("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801]);
    assert_mem("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
