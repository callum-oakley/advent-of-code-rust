use std::collections::HashMap;

use crate::freqs::Freqs;

fn parse(input: &str) -> impl Iterator<Item = (Vec<&str>, Vec<&str>)> + '_ {
    input.trim().lines().map(|line| {
        let (signal, output) = line.split_once('|').unwrap();
        (
            signal.trim().split(' ').collect(),
            output.trim().split(' ').collect(),
        )
    })
}

fn wires_to_freqs(wire_freqs: &HashMap<char, usize>, signal: &str) -> Vec<usize> {
    let mut res: Vec<usize> = signal.chars().map(|c| wire_freqs[&c]).collect();
    res.sort_unstable();
    res
}

// Across any 10 calibration signals, the same segments will always appear the same number of times.
// For example the top segment will always appear 8 times, while the bottom segment will always
// appear 7 times. It turns out that if we replace the wire labels with these frequencies of
// occurance on the standard wiring, each digit corresponds to a unique multiset of frequencies
// (freqs_to_digit below).
//
// Applying this same process to any given set of 10 calibration signals and comparing to the
// standard allows us to deduce how they are wired.
fn decode(input: &str) -> impl Iterator<Item = Vec<u8>> {
    let standard: HashMap<&str, u8> = HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("adbfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]);
    let wire_freqs = standard.keys().flat_map(|signal| signal.chars()).freqs();
    let freqs_to_digit: HashMap<Vec<usize>, u8> = standard
        .into_iter()
        .map(|(signal, digit)| (wires_to_freqs(&wire_freqs, signal), digit))
        .collect();

    parse(input).map(move |(signals, output)| {
        let wire_freqs = signals.iter().flat_map(|signal| signal.chars()).freqs();
        output
            .into_iter()
            .map(|signal| freqs_to_digit[&wires_to_freqs(&wire_freqs, signal)])
            .collect::<Vec<_>>()
    })
}

pub fn part1(input: &str) -> usize {
    decode(input)
        .flatten()
        .filter(|digit| [1, 4, 7, 8].contains(digit))
        .count()
}

pub fn part2(input: &str) -> u32 {
    decode(input)
        .flat_map(|digits| {
            digits
                .into_iter()
                .rev()
                .enumerate()
                .map(|(i, digit)| 10u32.pow(i.try_into().unwrap()) * u32::from(digit))
        })
        .sum()
}

pub fn tests() {
    let example = "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";
    assert_eq!(part1(example), 26);
    assert_eq!(part2(example), 61229);
}
