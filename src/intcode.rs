use crate::unbounded_vec::UnboundedVec;

/// A virtual machine that runs Intcode. Memory is an unbounded vec of 64 bit signed integers. If
/// you know what state a VM is in then call `input`, `output`, or `halt`, otherwise call `state`
/// and match on the result.
#[derive(Clone)]
pub struct VM {
    pub mem: UnboundedVec<i64>,
    ip: usize,
    base: i64,
}

/// The three states that a VM can be in once it pauses.
#[derive(PartialEq, Debug)]
pub enum State {
    /// Waiting for a call to `input`.
    Input,
    /// Waiting for a call to `output`.
    Output,
    /// Waiting for a call to `halt`.
    Halt,
}

impl VM {
    /// Construct a VM which will run the given Intcode program.
    pub fn new(prog: &str) -> Self {
        Self {
            mem: prog.split(',').map(|s| s.parse().unwrap()).collect(),
            ip: 0,
            base: 0,
        }
    }

    /// Drive the VM forward until it reaches an input, output, or halt instruction.
    pub fn state(&mut self) -> State {
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
                9 => {
                    self.base += *self.arg(1);
                    self.ip += 2;
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

    /// Input a value. Panic if the VM is not in `Input` state.
    pub fn input(&mut self, input: i64) {
        match self.state() {
            State::Input => {
                *self.arg(1) = input;
                self.ip += 2;
            }
            state => panic!("can't input when state is {state:?}"),
        }
    }

    /// Output a value. Panic if the VM is not in `Output` state.
    pub fn output(&mut self) -> i64 {
        match self.state() {
            State::Output => {
                let output = *self.arg(1);
                self.ip += 2;
                output
            }
            state => panic!("can't output when state is {state:?}"),
        }
    }

    /// Halt. Panic if the VM is not in `Halt` state. Good practice to call this when done to ensure
    /// no outputs are missed.
    pub fn halt(&mut self) {
        match self.state() {
            State::Halt => {}
            state => panic!("can't halt when state is {state:?}"),
        }
    }

    /// Get the value of the argument at offset `n` from the instruction pointer according to the
    /// specified mode.
    fn arg(&mut self, n: usize) -> &mut i64 {
        match self.mem[self.ip] / 10_i64.pow(1 + u32::try_from(n).unwrap()) % 10 {
            0 => {
                let i = usize::try_from(self.mem[self.ip + n]).unwrap();
                &mut self.mem[i]
            }
            1 => &mut self.mem[self.ip + n],
            2 => {
                let i = usize::try_from(self.mem[self.ip + n] + self.base).unwrap();
                &mut self.mem[i]
            }
            m => panic!("unsupported mode: {m}"),
        }
    }
}
