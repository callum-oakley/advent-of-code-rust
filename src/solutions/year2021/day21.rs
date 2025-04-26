use std::collections::HashMap;

use regex::Regex;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct State {
    position0: usize,
    position1: usize,
    score0: usize,
    score1: usize,
}

impl State {
    fn swap(self) -> Self {
        State {
            position0: self.position1,
            position1: self.position0,
            score0: self.score1,
            score1: self.score0,
        }
    }
}

fn parse(input: &str) -> State {
    let re = Regex::new(r"Player \d starting position: (\d+)").unwrap();
    let mut captures = re.captures_iter(input);
    State {
        position0: captures.next().unwrap()[1].parse().unwrap(),
        position1: captures.next().unwrap()[1].parse().unwrap(),
        score0: 0,
        score1: 0,
    }
}

pub fn part1(input: &str) -> usize {
    let mut state = parse(input);
    let mut dice = (1..=100).cycle();
    let mut rolls = 0;
    loop {
        let roll = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
        rolls += 3;
        state.position0 = crate::number_theory::wrap(state.position0 + roll, 1, 11);
        state.score0 += state.position0;
        if state.score0 >= 1000 {
            return state.score1 * rolls;
        }
        state = state.swap();
    }
}

pub fn part2(input: &str) -> usize {
    // The number of universes in which each player wins from the given state.
    fn game_inner(cache: &mut HashMap<State, (usize, usize)>, state: State) -> (usize, usize) {
        if state.score1 >= 21 {
            return (0, 1);
        }
        let mut res = (0, 0);
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let mut s = state;
                    s.position0 = crate::number_theory::wrap(s.position0 + a + b + c, 1, 11);
                    s.score0 += s.position0;
                    let wins = game(cache, s.swap());
                    res.0 += wins.1;
                    res.1 += wins.0;
                }
            }
        }
        res
    }

    fn game(cache: &mut HashMap<State, (usize, usize)>, state: State) -> (usize, usize) {
        if let Some(res) = cache.get(&state) {
            return *res;
        }
        let res = game_inner(cache, state);
        cache.insert(state, res);
        res
    }

    let wins = game(&mut HashMap::new(), parse(input));
    wins.0.max(wins.1)
}

pub fn tests() {
    let example = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
    assert_eq!(part1(example), 739_785);
    assert_eq!(part2(example), 444_356_092_776_315);
}
