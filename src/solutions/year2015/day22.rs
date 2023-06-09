use std::{cmp, collections::BTreeMap, iter};

use regex::Regex;

use crate::search;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Character {
    hp: i32,
    damage: i32,
    mana: i32,
}

impl Character {
    const PLAYER: Self = Self {
        hp: 50,
        damage: 0,
        mana: 500,
    };

    fn boss(input: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let mut stats = re.find_iter(input).map(|m| m.as_str().parse().unwrap());
        Self {
            hp: stats.next().unwrap(),
            damage: stats.next().unwrap(),
            mana: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Turn {
    Player,
    Boss,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    player: Character,
    boss: Character,
    effects: BTreeMap<Effect, i32>,
    turn: Turn,
    hard_mode: bool,
    mana_spent: i32,
}

struct Spell {
    cost: i32,
    damage: i32,
    healing: i32,
    effect: Option<(Effect, i32)>,
}

const SPELLS: [Spell; 5] = [
    Spell {
        cost: 53,
        damage: 4,
        healing: 0,
        effect: None,
    },
    Spell {
        cost: 73,
        damage: 2,
        healing: 2,
        effect: None,
    },
    Spell {
        cost: 113,
        damage: 0,
        healing: 0,
        effect: Some((Effect::Shield, 6)),
    },
    Spell {
        cost: 173,
        damage: 0,
        healing: 0,
        effect: Some((Effect::Poison, 6)),
    },
    Spell {
        cost: 229,
        damage: 0,
        healing: 0,
        effect: Some((Effect::Recharge, 5)),
    },
];

impl State {
    fn apply_effects(&mut self) {
        self.effects = self
            .effects
            .iter()
            .filter_map(|(effect, duration)| {
                match effect {
                    Effect::Shield => (),
                    Effect::Poison => self.boss.hp -= 3,
                    Effect::Recharge => self.player.mana += 101,
                };
                if *duration == 1 {
                    None
                } else {
                    Some((*effect, *duration - 1))
                }
            })
            .collect();
    }

    fn player_attack(&self, spell: &Spell) -> Self {
        let mut state = self.clone();
        state.boss.hp -= spell.damage;
        state.player.hp += spell.healing;
        if let Some((effect, duration)) = &spell.effect {
            state.effects.insert(*effect, *duration);
        }
        state.player.mana -= spell.cost;
        state.mana_spent += spell.cost;
        state.turn = Turn::Boss;
        state
    }

    fn boss_attack(&self) -> Self {
        let mut state = self.clone();
        if state.effects.contains_key(&Effect::Shield) {
            state.player.hp -= cmp::max(1, state.boss.damage - 7);
        } else {
            state.player.hp -= state.boss.damage;
        }
        state.turn = Turn::Player;
        state
    }
}

impl search::State for State {
    type HashKey = Self;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        let mut state = self.clone();
        if state.hard_mode && state.turn == Turn::Player {
            state.player.hp -= 1;
            if state.player.hp <= 0 {
                return Box::new(iter::empty());
            }
        }

        state.apply_effects();
        if state.player.hp <= 0 {
            return Box::new(iter::empty());
        } else if state.boss.hp <= 0 {
            return Box::new(iter::once(state));
        }

        match state.turn {
            Turn::Player => Box::new(SPELLS.iter().filter_map(move |spell| {
                if spell.cost <= state.player.mana
                    && spell
                        .effect
                        .map_or(true, |(effect, _)| !state.effects.contains_key(&effect))
                {
                    Some(state.player_attack(spell))
                } else {
                    None
                }
            })),
            Turn::Boss => Box::new(iter::once(state.boss_attack())),
        }
    }

    fn hash_key(&self) -> Self::HashKey {
        self.clone()
    }
}

impl search::OrdKey for State {
    type OrdKey = i32;

    fn ord_key(&self) -> Self::OrdKey {
        self.mana_spent
    }
}

fn part_(hard_mode: bool, input: &str) -> i32 {
    search::min_first(State {
        player: Character::PLAYER,
        boss: Character::boss(input),
        effects: BTreeMap::new(),
        turn: Turn::Player,
        hard_mode,
        mana_spent: 0,
    })
    .find(|state| state.boss.hp <= 0)
    .unwrap()
    .mana_spent
}

pub fn part1(input: &str) -> i32 {
    part_(false, input)
}

pub fn part2(input: &str) -> i32 {
    part_(true, input)
}
