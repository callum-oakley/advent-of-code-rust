use crate::{
    grid::{Point, Z},
    intcode::VM,
};

fn in_beam(mut vm: VM, pos: Point) -> bool {
    vm.input(pos.x.into());
    vm.input(pos.y.into());
    let res = vm.output() == 1;
    vm.halt();
    res
}

pub fn part1(input: &str) -> usize {
    let vm = VM::new(input);
    let mut res = 0;
    for y in 0..50 {
        for x in 0..50 {
            if in_beam(vm.clone(), Point { y, x }) {
                res += 1;
            }
        }
    }
    res
}

pub fn part2(input: &str) -> i32 {
    let vm = VM::new(input);
    let mut pos = Z;
    loop {
        if !in_beam(vm.clone(), pos + Point { y: 0, x: 99 }) {
            pos.y += 1;
        } else if !in_beam(vm.clone(), pos + Point { y: 99, x: 0 }) {
            pos.x += 1;
        } else {
            return 10000 * pos.x + pos.y;
        }
    }
}
