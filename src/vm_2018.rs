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

impl From<u8> for Mode {
    fn from(c: u8) -> Self {
        match c {
            b'r' => Mode::Register,
            b'i' => Mode::Immediate,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Op {
    category: Category,
    mode_a: Mode,
    mode_b: Mode,
}

impl<'a> From<&'a str> for Op {
    fn from(s: &'a str) -> Self {
        let s = s.as_bytes();
        match &s[0..2] {
            b"ad" => Op {
                category: Category::Add,
                mode_a: Mode::Register,
                mode_b: s[3].into(),
            },
            b"mu" => Op {
                category: Category::Mul,
                mode_a: Mode::Register,
                mode_b: s[3].into(),
            },
            b"ba" => Op {
                category: Category::Ban,
                mode_a: Mode::Register,
                mode_b: s[3].into(),
            },
            b"bo" => Op {
                category: Category::Bor,
                mode_a: Mode::Register,
                mode_b: s[3].into(),
            },
            b"se" => Op {
                category: Category::Set,
                mode_a: s[3].into(),
                mode_b: Mode::Register,
            },
            b"gt" => Op {
                category: Category::Gt,
                mode_a: s[2].into(),
                mode_b: s[3].into(),
            },
            b"eq" => Op {
                category: Category::Eq,
                mode_a: s[2].into(),
                mode_b: s[3].into(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    pub op: Op,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[rustfmt::skip]
pub const OPS: [Op; 16] = [
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

pub fn apply(i: Instruction, regs: &mut [usize]) {
    fn arg(regs: &[usize], mode: Mode, x: usize) -> usize {
        match mode {
            Mode::Register => regs[x],
            Mode::Immediate => x,
        }
    }
    regs[i.c] = match i.op.category {
        Category::Add => arg(regs, i.op.mode_a, i.a) + arg(regs, i.op.mode_b, i.b),
        Category::Mul => arg(regs, i.op.mode_a, i.a) * arg(regs, i.op.mode_b, i.b),
        Category::Ban => arg(regs, i.op.mode_a, i.a) & arg(regs, i.op.mode_b, i.b),
        Category::Bor => arg(regs, i.op.mode_a, i.a) | arg(regs, i.op.mode_b, i.b),
        Category::Set => arg(regs, i.op.mode_a, i.a),
        Category::Gt => usize::from(arg(regs, i.op.mode_a, i.a) > arg(regs, i.op.mode_b, i.b)),
        Category::Eq => usize::from(arg(regs, i.op.mode_a, i.a) == arg(regs, i.op.mode_b, i.b)),
    };
}

fn parse_instruction(input: &str) -> Instruction {
    let mut words = input.split_whitespace();
    Instruction {
        op: words.next().unwrap().into(),
        a: words.next().unwrap().parse().unwrap(),
        b: words.next().unwrap().parse().unwrap(),
        c: words.next().unwrap().parse().unwrap(),
    }
}

pub fn parse(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();
    (
        lines
            .next()
            .unwrap()
            .strip_prefix("#ip ")
            .unwrap()
            .parse()
            .unwrap(),
        lines.map(parse_instruction).collect(),
    )
}
