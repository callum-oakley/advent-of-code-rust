use std::io;

use advent_of_code::{get_input, intcode::VM, solutions::year2019::day17::play};
use anyhow::Result;

fn main() -> Result<()> {
    let mut vm = VM::new(get_input(2019, 17).trim_end_matches('\n'));
    vm.mem[0] = 2;
    let dust = play(&mut vm, io::stdin(), io::stdout())?;
    println!("dust: {dust:?}");
    Ok(())
}
