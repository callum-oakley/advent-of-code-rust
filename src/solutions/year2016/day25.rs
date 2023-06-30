const SIGNALS_TO_CHECK: i32 = 10;

type Reg = usize;

fn reg(s: &str) -> Reg {
    s.chars().next().unwrap() as usize - 'a' as usize
}

#[derive(PartialEq, Eq, Hash)]
enum Arg {
    Lit(i32),
    Reg(Reg),
}

impl Arg {
    fn value(&self, regs: &[i32]) -> i32 {
        match self {
            Arg::Lit(lit) => *lit,
            Arg::Reg(reg) => regs[*reg],
        }
    }
}

impl From<&str> for Arg {
    fn from(s: &str) -> Self {
        if let Ok(n) = s.parse() {
            Arg::Lit(n)
        } else {
            Arg::Reg(reg(s))
        }
    }
}

enum Instruction {
    Cpy(Arg, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Arg, Arg),
    Out(Arg),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let op = words.next();
            let x = words.next();
            let y = words.next();
            match op.unwrap() {
                "cpy" => Instruction::Cpy(x.unwrap().into(), reg(y.unwrap())),
                "inc" => Instruction::Inc(reg(x.unwrap())),
                "dec" => Instruction::Dec(reg(x.unwrap())),
                "jnz" => Instruction::Jnz(x.unwrap().into(), y.unwrap().into()),
                "out" => Instruction::Out(x.unwrap().into()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn generates_clock_signal(a: i32, instructions: &[Instruction]) -> bool {
    let mut signals = 0;

    let mut regs = [a, 0, 0, 0];
    let mut ip = 0;
    while ip < instructions.len() && signals < SIGNALS_TO_CHECK {
        match &instructions[ip] {
            Instruction::Cpy(x, y) => {
                regs[*y] = x.value(&regs);
            }
            Instruction::Inc(x) => {
                regs[*x] += 1;
            }
            Instruction::Dec(x) => {
                regs[*x] -= 1;
            }
            Instruction::Jnz(x, y) => {
                if x.value(&regs) != 0 {
                    let y = y.value(&regs);
                    if y < 0 {
                        ip -= usize::try_from(-y).unwrap();
                    } else {
                        ip += usize::try_from(y).unwrap();
                    }
                    continue;
                }
            }
            Instruction::Out(x) => {
                if x.value(&regs) != signals % 2 {
                    return false;
                }
                signals += 1;
            }
        }
        ip += 1;
    }

    true
}

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    for a in 1.. {
        if generates_clock_signal(a, &instructions) {
            return a;
        }
    }
    unreachable!()
}
