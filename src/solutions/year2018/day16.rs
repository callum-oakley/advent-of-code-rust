use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::vm_2018;

type Regs = [usize; 4];

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

struct Sample {
    before: Regs,
    instruction: Instruction,
    after: Regs,
}

fn apply(ops: [vm_2018::Op; 16], i: Instruction, regs: &mut Regs) {
    vm_2018::apply(
        vm_2018::Instruction {
            op: ops[i.opcode],
            a: i.a,
            b: i.b,
            c: i.c,
        },
        regs,
    );
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

fn parse_sample(input: &str) -> Sample {
    let mut ints = RE.find_iter(input);
    Sample {
        before: [
            ints.next().unwrap().as_str().parse().unwrap(),
            ints.next().unwrap().as_str().parse().unwrap(),
            ints.next().unwrap().as_str().parse().unwrap(),
            ints.next().unwrap().as_str().parse().unwrap(),
        ],
        instruction: Instruction {
            opcode: ints.next().unwrap().as_str().parse().unwrap(),
            a: ints.next().unwrap().as_str().parse().unwrap(),
            b: ints.next().unwrap().as_str().parse().unwrap(),
            c: ints.next().unwrap().as_str().parse().unwrap(),
        },
        after: [
            ints.next().unwrap().as_str().parse().unwrap(),
            ints.next().unwrap().as_str().parse().unwrap(),
            ints.next().unwrap().as_str().parse().unwrap(),
            ints.next().unwrap().as_str().parse().unwrap(),
        ],
    }
}

fn parse_instruction(input: &str) -> Instruction {
    let mut ints = RE.find_iter(input);
    Instruction {
        opcode: ints.next().unwrap().as_str().parse().unwrap(),
        a: ints.next().unwrap().as_str().parse().unwrap(),
        b: ints.next().unwrap().as_str().parse().unwrap(),
        c: ints.next().unwrap().as_str().parse().unwrap(),
    }
}

fn parse(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let (samples, program) = input.split_once("\n\n\n\n").unwrap();
    (
        samples.split("\n\n").map(parse_sample).collect(),
        program.lines().map(parse_instruction).collect(),
    )
}

fn consistent(sample: &Sample) -> impl Iterator<Item = vm_2018::Op> + '_ {
    vm_2018::OPS.into_iter().filter(|&op| {
        let mut before = sample.before;
        apply([op; 16], sample.instruction, &mut before);
        before == sample.after
    })
}

// Reduce the one-many map of possibilities to a one-one map by process of elimination
fn eliminate(mut possible: [HashSet<vm_2018::Op>; 16]) -> [Option<vm_2018::Op>; 16] {
    for i in 0..16 {
        if possible[i].len() == 1 {
            let op = *possible[i].iter().next().unwrap();
            for ops in &mut possible {
                ops.remove(&op);
            }
            let mut res = eliminate(possible);
            res[i] = Some(op);
            return res;
        }
    }
    [None; 16]
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .0
        .into_iter()
        .filter(|sample| consistent(sample).count() >= 3)
        .count()
}

pub fn part2(input: &str) -> usize {
    let (samples, program) = parse(input);

    let mut possible: [HashSet<vm_2018::Op>; 16] = std::array::from_fn(|_| HashSet::new());
    for sample in samples {
        for op in consistent(&sample) {
            possible[sample.instruction.opcode].insert(op);
        }
    }

    let ops = eliminate(possible);
    let ops = ops.map(std::option::Option::unwrap);

    let mut regs = [0; 4];
    for instruction in program {
        apply(ops, instruction, &mut regs);
    }
    regs[0]
}

pub fn tests() {
    let example = "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]";
    assert_eq!(
        consistent(&parse_sample(example)).collect::<Vec<_>>(),
        vec![vm_2018::OPS[1], vm_2018::OPS[2], vm_2018::OPS[9]],
    );
}
