use std::io;

use advent_of_code::{get_input, solutions::year2019::day25::play};
use anyhow::Result;

fn main() -> Result<()> {
    play(
        get_input(2019, 25).trim_end_matches('\n'),
        io::stdin(),
        io::stdout(),
    )?;
    Ok(())
}
