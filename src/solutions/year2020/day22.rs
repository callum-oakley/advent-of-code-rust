use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let (a, b) = input.split_once("\n\n").unwrap();
    (
        a.trim()
            .trim_start_matches("Player 1:")
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
        b.trim()
            .trim_start_matches("Player 2:")
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
    )
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card)
        .sum()
}

pub fn part1(input: &str) -> usize {
    let (mut deck0, mut deck1) = parse(input);
    loop {
        let card0 = deck0.pop_front().unwrap();
        let card1 = deck1.pop_front().unwrap();

        if card0 > card1 {
            deck0.push_back(card0);
            deck0.push_back(card1);
        } else {
            deck1.push_back(card1);
            deck1.push_back(card0);
        }

        if deck0.is_empty() {
            return score(&deck1);
        }
        if deck1.is_empty() {
            return score(&deck0);
        }
    }
}

pub fn part2(input: &str) -> usize {
    struct Game {
        score: usize,
        winner: usize,
    }

    fn game(mut decks: (VecDeque<usize>, VecDeque<usize>)) -> Game {
        let mut seen = HashSet::new();
        loop {
            if seen.contains(&decks) {
                return Game {
                    score: score(&decks.0),
                    winner: 0,
                };
            }
            seen.insert(decks.clone());

            let card0 = decks.0.pop_front().unwrap();
            let card1 = decks.1.pop_front().unwrap();

            let zero_wins = if decks.0.len() >= card0 && decks.1.len() >= card1 {
                game((
                    decks.0.iter().take(card0).copied().collect(),
                    decks.1.iter().take(card1).copied().collect(),
                ))
                .winner
                    == 0
            } else {
                card0 > card1
            };

            if zero_wins {
                decks.0.push_back(card0);
                decks.0.push_back(card1);
            } else {
                decks.1.push_back(card1);
                decks.1.push_back(card0);
            }

            if decks.0.is_empty() {
                return Game {
                    score: score(&decks.1),
                    winner: 1,
                };
            }
            if decks.1.is_empty() {
                return Game {
                    score: score(&decks.0),
                    winner: 0,
                };
            }
        }
    }

    game(parse(input)).score
}

pub fn tests() {
    let example = "
        Player 1: 9 2 6 3 1

        Player 2: 5 8 4 7 10
    ";
    assert_eq!(part1(example), 306);
    assert_eq!(part2(example), 291);
}
