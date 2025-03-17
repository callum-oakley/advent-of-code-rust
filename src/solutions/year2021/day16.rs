use std::fmt::Write;

struct Bits<'a> {
    inner: &'a str,
}

impl Bits<'_> {
    /// Take `n` bits from `self` and return them as a new `Bits`.
    fn take(&mut self, n: usize) -> Self {
        let res = Self {
            inner: &self.inner[..n],
        };
        self.inner = &self.inner[n..];
        res
    }

    /// Take `n` bits from `self` and parse them as a `usize`.
    fn read(&mut self, n: usize) -> usize {
        usize::from_str_radix(self.take(n).inner, 2).unwrap()
    }
}

enum Op {
    Sum,
    Prod,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

enum Payload {
    Literal(usize),
    Operator(Op, Vec<Packet>),
}

struct Packet {
    version: usize,
    payload: Payload,
}

fn parse_literal(bits: &mut Bits) -> usize {
    let mut more = bits.read(1);
    let mut literal = bits.read(4);
    while more == 1 {
        more = bits.read(1);
        literal = (literal << 4) + bits.read(4);
    }
    literal
}

fn parse_args(bits: &mut Bits) -> Vec<Packet> {
    let mut args = Vec::new();
    let length_type_id = bits.read(1);
    if length_type_id == 0 {
        let bits_count = bits.read(15);
        let mut bits = bits.take(bits_count);
        while !bits.inner.is_empty() {
            args.push(parse_packet(&mut bits));
        }
    } else {
        let args_count = bits.read(11);
        for _ in 0..args_count {
            args.push(parse_packet(bits));
        }
    }
    args
}

fn parse_packet(bits: &mut Bits) -> Packet {
    let version = bits.read(3);
    let type_id = bits.read(3);
    Packet {
        version,
        payload: match type_id {
            0 => Payload::Operator(Op::Sum, parse_args(bits)),
            1 => Payload::Operator(Op::Prod, parse_args(bits)),
            2 => Payload::Operator(Op::Min, parse_args(bits)),
            3 => Payload::Operator(Op::Max, parse_args(bits)),
            4 => Payload::Literal(parse_literal(bits)),
            5 => Payload::Operator(Op::Greater, parse_args(bits)),
            6 => Payload::Operator(Op::Less, parse_args(bits)),
            7 => Payload::Operator(Op::Equal, parse_args(bits)),
            _ => unreachable!(),
        },
    }
}

fn parse(input: &str) -> Packet {
    let mut bits = String::new();
    for c in input.chars() {
        write!(bits, "{:0>4b}", c.to_digit(16).unwrap()).unwrap();
    }
    parse_packet(&mut Bits { inner: &bits })
}

pub fn part1(input: &str) -> usize {
    fn version_sum(packet: &Packet) -> usize {
        packet.version
            + match &packet.payload {
                Payload::Literal(_) => 0,
                Payload::Operator(_, args) => args.iter().map(version_sum).sum(),
            }
    }
    version_sum(&parse(input))
}

pub fn part2(input: &str) -> usize {
    fn eval(packet: &Packet) -> usize {
        match &packet.payload {
            Payload::Literal(v) => *v,
            Payload::Operator(op, args) => match op {
                Op::Sum => args.iter().map(eval).sum(),
                Op::Prod => args.iter().map(eval).product(),
                Op::Min => args.iter().map(eval).min().unwrap(),
                Op::Max => args.iter().map(eval).max().unwrap(),
                Op::Greater => (eval(&args[0]) > eval(&args[1])).into(),
                Op::Less => (eval(&args[0]) < eval(&args[1])).into(),
                Op::Equal => (eval(&args[0]) == eval(&args[1])).into(),
            },
        }
    }
    eval(&parse(input))
}

pub fn tests() {
    assert_eq!(part1("8A004A801A8002F478"), 16);
    assert_eq!(part1("620080001611562C8802118E34"), 12);
    assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
    assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    assert_eq!(part2("C200B40A82"), 3);
    assert_eq!(part2("04005AC33890"), 54);
    assert_eq!(part2("880086C3E88112"), 7);
    assert_eq!(part2("CE00C43D881120"), 9);
    assert_eq!(part2("D8005AC2A8F0"), 1);
    assert_eq!(part2("F600BC2D8F"), 0);
    assert_eq!(part2("9C005AC2F8F0"), 0);
    assert_eq!(part2("9C0141080250320F1802104A08"), 1);
}
