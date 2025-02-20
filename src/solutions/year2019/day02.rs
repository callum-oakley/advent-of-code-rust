use crate::{intcode::VM, unbounded_vec::UnboundedVec};

fn run(mut vm: VM, noun: i64, verb: i64) -> i64 {
    vm.mem[1] = noun;
    vm.mem[2] = verb;
    vm.halt();
    vm.mem[0]
}

pub fn part1(input: &str) -> i64 {
    run(VM::new(input), 12, 2)
}

pub fn part2(input: &str) -> i64 {
    let vm = VM::new(input);
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
    fn assert_mem(input: &str, expected: &UnboundedVec<i64>) {
        let mut vm = VM::new(input);
        vm.halt();
        assert_eq!(&vm.mem, expected);
    }

    assert_mem(
        "1,9,10,3,2,3,11,0,99,30,40,50",
        &UnboundedVec::from_iter([3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
    );
    assert_mem("1,0,0,0,99", &UnboundedVec::from_iter([2, 0, 0, 0, 99]));
    assert_mem("2,3,0,3,99", &UnboundedVec::from_iter([2, 3, 0, 6, 99]));
    assert_mem(
        "2,4,4,5,99,0",
        &UnboundedVec::from_iter([2, 4, 4, 5, 99, 9801]),
    );
    assert_mem(
        "1,1,1,4,99,5,6,0,99",
        &UnboundedVec::from_iter([30, 1, 1, 4, 2, 5, 6, 0, 99]),
    );
}
