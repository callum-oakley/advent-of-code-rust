use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Category {
    Add,
    Mul,
    Ban,
    Bor,
    Set,
    Gt,
    Eq,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Mode {
    Register,
    Immediate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Op {
    category: Category,
    mode_a: Mode,
    mode_b: Mode,
}

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

#[rustfmt::skip]
const OPS: [Op; 16] = [
    Op { category: Category::Add, mode_a: Mode::Register,  mode_b: Mode::Register  }, // addr
    Op { category: Category::Add, mode_a: Mode::Register,  mode_b: Mode::Immediate }, // addi
    Op { category: Category::Mul, mode_a: Mode::Register,  mode_b: Mode::Register  }, // mulr
    Op { category: Category::Mul, mode_a: Mode::Register,  mode_b: Mode::Immediate }, // muli
    Op { category: Category::Ban, mode_a: Mode::Register,  mode_b: Mode::Register  }, // banr
    Op { category: Category::Ban, mode_a: Mode::Register,  mode_b: Mode::Immediate }, // bani
    Op { category: Category::Bor, mode_a: Mode::Register,  mode_b: Mode::Register  }, // borr
    Op { category: Category::Bor, mode_a: Mode::Register,  mode_b: Mode::Immediate }, // bori
    Op { category: Category::Set, mode_a: Mode::Register,  mode_b: Mode::Register  }, // setr
    Op { category: Category::Set, mode_a: Mode::Immediate, mode_b: Mode::Immediate }, // seti
    Op { category: Category::Gt,  mode_a: Mode::Immediate, mode_b: Mode::Register  }, // gtir
    Op { category: Category::Gt,  mode_a: Mode::Register,  mode_b: Mode::Immediate }, // gtri
    Op { category: Category::Gt,  mode_a: Mode::Register,  mode_b: Mode::Register  }, // gtrr
    Op { category: Category::Eq,  mode_a: Mode::Immediate, mode_b: Mode::Register  }, // eqir
    Op { category: Category::Eq,  mode_a: Mode::Register,  mode_b: Mode::Immediate }, // eqri
    Op { category: Category::Eq,  mode_a: Mode::Register,  mode_b: Mode::Register  }, // eqrr
];

fn apply(ops: [Op; 16], i: Instruction, mut regs: Regs) -> Regs {
    fn arg(regs: &Regs, mode: Mode, x: usize) -> usize {
        match mode {
            Mode::Register => regs[x],
            Mode::Immediate => x,
        }
    }
    let op = ops[i.opcode];
    regs[i.c] = match op.category {
        Category::Add => arg(&regs, op.mode_a, i.a) + arg(&regs, op.mode_b, i.b),
        Category::Mul => arg(&regs, op.mode_a, i.a) * arg(&regs, op.mode_b, i.b),
        Category::Ban => arg(&regs, op.mode_a, i.a) & arg(&regs, op.mode_b, i.b),
        Category::Bor => arg(&regs, op.mode_a, i.a) | arg(&regs, op.mode_b, i.b),
        Category::Set => arg(&regs, op.mode_a, i.a),
        Category::Gt => usize::from(arg(&regs, op.mode_a, i.a) > arg(&regs, op.mode_b, i.b)),
        Category::Eq => usize::from(arg(&regs, op.mode_a, i.a) == arg(&regs, op.mode_b, i.b)),
    };
    regs
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

fn consistent(sample: &Sample) -> impl Iterator<Item = Op> + '_ {
    OPS.into_iter()
        .filter(|&op| apply([op; 16], sample.instruction, sample.before) == sample.after)
}

// Reduce the one-many map of possibilities to a one-one map by process of elimination
fn eliminate(mut possible: [HashSet<Op>; 16]) -> [Option<Op>; 16] {
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

    let mut possible: [HashSet<Op>; 16] = std::array::from_fn(|_| HashSet::new());
    for sample in samples {
        for op in consistent(&sample) {
            possible[sample.instruction.opcode].insert(op);
        }
    }

    let ops = eliminate(possible);
    let ops = ops.map(std::option::Option::unwrap);

    let mut regs = [0; 4];
    for instruction in program {
        regs = apply(ops, instruction, regs);
    }
    regs[0]
}

pub fn tests() {
    let example = "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]";
    assert_eq!(
        consistent(&parse_sample(example)).collect::<Vec<_>>(),
        vec![OPS[1], OPS[2], OPS[9]],
    );
}
