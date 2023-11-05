const MEM_SIZE: usize = 1024;

#[derive(Clone)]
pub struct VM {
    pub mem: [i32; MEM_SIZE],
    pub ip: usize,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Input,
    Output,
    Halt,
}

impl VM {
    pub fn new(prog: &str) -> Self {
        let mut mem = [0; MEM_SIZE];
        for (i, s) in prog.split(',').enumerate() {
            mem[i] = s.parse().unwrap();
        }
        Self { mem, ip: 0 }
    }

    fn run(&mut self) -> State {
        loop {
            match self.mem[self.ip] % 100 {
                1 => {
                    *self.arg(3) = *self.arg(1) + *self.arg(2);
                    self.ip += 4;
                }
                2 => {
                    *self.arg(3) = *self.arg(1) * *self.arg(2);
                    self.ip += 4;
                }
                3 => {
                    return State::Input;
                }
                4 => {
                    return State::Output;
                }
                5 => {
                    if *self.arg(1) == 0 {
                        self.ip += 3;
                    } else {
                        self.ip = usize::try_from(*self.arg(2)).unwrap();
                    }
                }
                6 => {
                    if *self.arg(1) == 0 {
                        self.ip = usize::try_from(*self.arg(2)).unwrap();
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    *self.arg(3) = (*self.arg(1) < *self.arg(2)).into();
                    self.ip += 4;
                }
                8 => {
                    *self.arg(3) = (*self.arg(1) == *self.arg(2)).into();
                    self.ip += 4;
                }
                99 => {
                    return State::Halt;
                }
                op => {
                    panic!("unsupported opcode: {op}");
                }
            }
        }
    }

    pub fn input(&mut self, input: i32) {
        match self.run() {
            State::Input => {
                *self.arg(1) = input;
                self.ip += 2;
            }
            state => panic!("can't input when state is {state:?}"),
        }
    }

    pub fn output(&mut self) -> i32 {
        match self.run() {
            State::Output => {
                let output = *self.arg(1);
                self.ip += 2;
                output
            }
            state => panic!("can't output when state is {state:?}"),
        }
    }

    pub fn halt(&mut self) {
        match self.run() {
            State::Halt => {}
            state => panic!("can't halt when state is {state:?}"),
        }
    }

    fn arg(&mut self, n: usize) -> &mut i32 {
        match self.mem[self.ip] / 10_i32.pow(1 + u32::try_from(n).unwrap()) % 10 {
            0 => &mut self.mem[usize::try_from(self.mem[self.ip + n]).unwrap()],
            1 => &mut self.mem[self.ip + n],
            m => panic!("unsupported mode: {m}"),
        }
    }
}
