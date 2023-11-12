use std::{thread::sleep, time::Duration};

use advent_of_code::{get_input, solutions::year2019::day13::play};

fn main() {
    println!("\u{009B}?25l\u{009B}2J");
    play(get_input(2019, 13).trim_end_matches('\n'), |x, y, z| {
        if x == -1 && y == 0 {
            println!("\u{009B}1;2H{z}\u{0007}");
        } else {
            println!(
                "\u{009B}{};{}H{}",
                y + 2,
                x + 2,
                match z {
                    0 => ' ',
                    1 => '#',
                    2 => '=',
                    3 => '-',
                    4 => 'o',
                    _ => unreachable!(),
                }
            );
        }
        sleep(Duration::from_millis(5));
    });
    println!("\u{009B}?25h\u{009B}28;0H");
}
