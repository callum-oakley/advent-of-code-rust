use std::io;

use advent_of_code::{get_input, intcode::VM, solutions::year2019::day21::play};
use anyhow::Result;

fn main() -> Result<()> {
    let mut vm = VM::new(get_input(2019, 21).trim_end_matches('\n'));
    let damage = play(&mut vm, io::stdin(), io::stdout())?;
    println!("damage: {damage:?}");
    Ok(())
}
