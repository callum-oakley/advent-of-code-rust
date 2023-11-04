const MEM_SIZE: usize = 1024;

pub type Memory = [usize; MEM_SIZE];

#[derive(Clone)]
pub struct VM {
    pub mem: Memory,
    pub ip: usize,
}

pub fn parse(input: &str) -> VM {
    let mut mem = [0; MEM_SIZE];
    for (i, s) in input.split(',').enumerate() {
        mem[i] = s.parse().unwrap();
    }
    VM { mem, ip: 0 }
}

impl VM {
    pub fn run(&mut self) {
        loop {
            let op = self.mem[self.ip];
            let a = self.mem[self.ip + 1];
            let b = self.mem[self.ip + 2];
            let c = self.mem[self.ip + 3];
            match op {
                1 => {
                    self.mem[c] = self.mem[a] + self.mem[b];
                    self.ip += 4;
                }
                2 => {
                    self.mem[c] = self.mem[a] * self.mem[b];
                    self.ip += 4;
                }
                99 => {
                    return;
                }
                _ => {
                    panic!("unsupported opcode: {op}");
                }
            }
        }
    }
}
