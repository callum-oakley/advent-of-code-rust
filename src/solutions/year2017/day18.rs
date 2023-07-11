use std::collections::VecDeque;

fn reg(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

#[derive(Clone, Copy)]
enum Arg {
    Lit(i64),
    Reg(usize),
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

#[derive(Clone)]
enum Instruction {
    Snd(Arg),
    Rcv(usize),
    Set(usize, Arg),
    Add(usize, Arg),
    Mul(usize, Arg),
    Mod(usize, Arg),
    Jgz(Arg, Arg),
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
                "snd" => Instruction::Snd(x.unwrap().into()),
                "rcv" => Instruction::Rcv(reg(x.unwrap())),
                "set" => Instruction::Set(reg(x.unwrap()), y.unwrap().into()),
                "add" => Instruction::Add(reg(x.unwrap()), y.unwrap().into()),
                "mul" => Instruction::Mul(reg(x.unwrap()), y.unwrap().into()),
                "mod" => Instruction::Mod(reg(x.unwrap()), y.unwrap().into()),
                "jgz" => Instruction::Jgz(x.unwrap().into(), y.unwrap().into()),
                _ => unreachable!(),
            }
        })
        .collect()
}

struct Machine<'a> {
    ip: usize,
    regs: [i64; 16],
    instructions: &'a [Instruction],
}

fn offset(ip: usize, n: i64) -> usize {
    if n < 0 {
        ip - usize::try_from(-n).unwrap()
    } else {
        ip + usize::try_from(n).unwrap()
    }
}

impl<'a> Machine<'a> {
    fn new(pid: i64, instructions: &'a [Instruction]) -> Self {
        let mut regs = [0; 16];
        regs[reg("p")] = pid;
        Machine {
            ip: 0,
            regs,
            instructions,
        }
    }

    fn value(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Lit(lit) => lit,
            Arg::Reg(reg) => self.regs[reg],
        }
    }

    // Run until reaching a snd or rcv and then return to let the caller decide
    // what to do.
    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            match &self.instructions[self.ip] {
                Instruction::Snd(_) | Instruction::Rcv(_) => {
                    self.ip += 1;
                    return;
                }
                Instruction::Set(x, y) => {
                    self.regs[*x] = self.value(*y);
                }
                Instruction::Add(x, y) => {
                    self.regs[*x] += self.value(*y);
                }
                Instruction::Mul(x, y) => {
                    self.regs[*x] *= self.value(*y);
                }
                Instruction::Mod(x, y) => {
                    self.regs[*x] %= self.value(*y);
                }
                Instruction::Jgz(x, y) => {
                    if self.value(*x) > 0 {
                        self.ip = offset(self.ip, self.value(*y));
                        continue;
                    }
                }
            }
            self.ip += 1;
        }
    }

    fn resume_from_rcv(&mut self, val: i64) {
        let Instruction::Rcv(x) = self.instructions[self.ip - 1] else {
            panic!("tried to resume from rcv but not in rcv state");
        };
        self.regs[x] = val;
        self.run();
    }
}

pub fn part1(input: &str) -> i64 {
    let instructions = parse(input);

    let mut m = Machine::new(0, &instructions);
    m.run();

    let mut res = 0;

    loop {
        match m.instructions[m.ip - 1] {
            Instruction::Snd(x) => {
                res = m.value(x);
                m.run();
            }
            Instruction::Rcv(x) => {
                if m.regs[x] != 0 {
                    break;
                }
                m.run();
            }
            _ => unreachable!(),
        }
    }

    res
}

pub fn part2(input: &str) -> usize {
    let instructions = parse(input);

    let mut m_0 = Machine::new(0, &instructions);
    let mut m_1 = Machine::new(1, &instructions);
    m_0.run();
    m_1.run();

    let mut inbox_0 = VecDeque::new();
    let mut inbox_1 = VecDeque::new();

    let mut res = 0;

    loop {
        match (&m_0.instructions[m_0.ip - 1], &m_1.instructions[m_1.ip - 1]) {
            (Instruction::Rcv(_), _) if !inbox_0.is_empty() => {
                m_0.resume_from_rcv(inbox_0.pop_front().unwrap());
            }
            (_, Instruction::Rcv(_)) if !inbox_1.is_empty() => {
                m_1.resume_from_rcv(inbox_1.pop_front().unwrap());
            }
            (Instruction::Snd(x), _) => {
                inbox_1.push_back(m_0.value(*x));
                m_0.run();
            }
            (_, Instruction::Snd(x)) => {
                res += 1;
                inbox_0.push_back(m_1.value(*x));
                m_1.run();
            }
            _ => break,
        }
    }

    res
}

pub fn tests() {
    assert_eq!(
        part1(
            "set a 1
             add a 2
             mul a a
             mod a 5
             snd a
             set a 0
             rcv a
             jgz a -1
             set a 1
             jgz a -2",
        ),
        4,
    );
    assert_eq!(
        part2(
            "snd 1
             snd 2
             snd p
             rcv a
             rcv b
             rcv c
             rcv d",
        ),
        3,
    );
}
