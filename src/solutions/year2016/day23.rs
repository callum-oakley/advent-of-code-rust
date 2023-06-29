type Reg = usize;

fn reg(s: &str) -> Reg {
    s.chars().next().unwrap() as usize - 'a' as usize
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

    fn reg(&self) -> usize {
        let Arg::Reg(reg) = self else {
            panic!("arg isn't a register");
        };
        *reg
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
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Tgl(Arg),
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match self {
            Instruction::Inc(x) => Instruction::Dec(*x),
            Instruction::Dec(x) | Instruction::Tgl(x) => Instruction::Inc(*x),
            Instruction::Jnz(x, y) => Instruction::Cpy(*x, *y),
            Instruction::Cpy(x, y) => Instruction::Jnz(*x, *y),
        }
    }
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
                "cpy" => Instruction::Cpy(x.unwrap().into(), y.unwrap().into()),
                "inc" => Instruction::Inc(x.unwrap().into()),
                "dec" => Instruction::Dec(x.unwrap().into()),
                "jnz" => Instruction::Jnz(x.unwrap().into(), y.unwrap().into()),
                "tgl" => Instruction::Tgl(x.unwrap().into()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn offset(ip: usize, n: i32) -> usize {
    if n < 0 {
        ip - usize::try_from(-n).unwrap()
    } else {
        ip + usize::try_from(n).unwrap()
    }
}

fn part_(a: i32, optimize: bool, input: &str) -> i32 {
    let mut instructions = parse(input);

    let mut regs = [a, 0, 0, 0];
    let mut ip = 0;
    while ip < instructions.len() {
        if optimize && ip == 2 {
            // Instructions 2 to 10 multipy a by b (slowly!)
            regs[reg("a")] *= regs[reg("b")];
            ip = 10;
            continue;
        }

        match &instructions[ip] {
            Instruction::Cpy(x, y) => {
                regs[y.reg()] = x.value(&regs);
            }
            Instruction::Inc(x) => {
                regs[x.reg()] += 1;
            }
            Instruction::Dec(x) => {
                regs[x.reg()] -= 1;
            }
            Instruction::Jnz(x, y) => {
                if x.value(&regs) != 0 {
                    ip = offset(ip, y.value(&regs));
                    continue;
                }
            }
            Instruction::Tgl(x) => {
                let y = offset(ip, x.value(&regs));
                if y < instructions.len() {
                    instructions[y].toggle();
                }
            }
        }
        ip += 1;
    }
    regs[reg("a")]
}

pub fn part1(input: &str) -> i32 {
    part_(7, true, input)
}

pub fn part2(input: &str) -> i32 {
    part_(12, true, input)
}

pub fn tests() {
    let example = "cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a";
    assert_eq!(part_(0, false, example), 3);
}
