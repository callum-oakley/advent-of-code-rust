use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::part::Part;

fn reg(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

#[derive(Clone)]
enum Arg {
    Lit(i64),
    Reg(usize),
}

impl Arg {
    fn value(&self, registers: &[i64]) -> i64 {
        match *self {
            Arg::Lit(lit) => lit,
            Arg::Reg(reg) => registers[reg],
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

fn offset(ip: usize, n: i64) -> usize {
    if n < 0 {
        ip - usize::try_from(-n).unwrap()
    } else {
        ip + usize::try_from(n).unwrap()
    }
}

fn run(
    part: Part,
    pid: i64,
    tx: &Sender<i64>,
    rx: &Receiver<i64>,
    instructions: &[Instruction],
) -> i64 {
    let mut registers = [0; 26];
    registers[reg("p")] = pid;

    let mut ip = 0;
    let mut res = 0;

    while ip < instructions.len() {
        match &instructions[ip] {
            Instruction::Snd(x) => match part {
                Part::One => {
                    res = x.value(&registers);
                }
                Part::Two => {
                    res += 1;
                    tx.send(x.value(&registers)).unwrap();
                }
            },
            Instruction::Rcv(x) => match part {
                Part::One => {
                    if registers[*x] != 0 {
                        break;
                    }
                }
                Part::Two => {
                    if let Ok(val) = rx.recv_timeout(Duration::from_millis(1)) {
                        registers[*x] = val;
                    } else {
                        // deadlock
                        break;
                    }
                }
            },
            Instruction::Set(x, y) => {
                registers[*x] = y.value(&registers);
            }
            Instruction::Add(x, y) => {
                registers[*x] += y.value(&registers);
            }
            Instruction::Mul(x, y) => {
                registers[*x] *= y.value(&registers);
            }
            Instruction::Mod(x, y) => {
                registers[*x] %= y.value(&registers);
            }
            Instruction::Jgz(x, y) => {
                if x.value(&registers) > 0 {
                    ip = offset(ip, y.value(&registers));
                    continue;
                }
            }
        }
        ip += 1;
    }

    res
}

pub fn part1(input: &str) -> i64 {
    let (tx, rx) = channel();
    run(Part::One, 0, &tx, &rx, &parse(input))
}

pub fn part2(input: &str) -> i64 {
    let instructions = parse(input);

    let (tx_a, rx_a) = channel();
    let (tx_b, rx_b) = channel();

    let prog_1 = {
        let instructions = instructions.clone();
        thread::spawn(move || run(Part::Two, 1, &tx_b, &rx_a, &instructions))
    };

    run(Part::Two, 0, &tx_a, &rx_b, &instructions);

    prog_1.join().unwrap()
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
