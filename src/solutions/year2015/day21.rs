use std::{cmp, mem};

use regex::Regex;

use crate::combinatorics::combination;

#[derive(Clone, Copy)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn loadouts() -> Vec<Vec<Self>> {
        let shop = [
            8, 4, 0, 10, 5, 0, 25, 6, 0, 40, 7, 0, 74, 8, 0, 13, 0, 1, 31, 0, 2, 53, 0, 3, 75, 0,
            4, 102, 0, 5, 25, 1, 0, 50, 2, 0, 100, 3, 0, 20, 0, 1, 40, 0, 2, 80, 0, 3,
        ];
        let items = shop
            .chunks(3)
            .map(|item| Self {
                cost: item[0],
                damage: item[1],
                armor: item[2],
            })
            .collect::<Vec<Self>>();

        let weapon_choices = combination(1, &items[0..5]).collect::<Vec<_>>();
        let armor_choices = combination(0, &items[5..10])
            .chain(combination(1, &items[5..10]))
            .collect::<Vec<_>>();
        let ring_choices = combination(0, &items[10..])
            .chain(combination(1, &items[10..]))
            .chain(combination(2, &items[10..]))
            .collect::<Vec<_>>();

        let mut res = Vec::new();
        for weapon_choice in &weapon_choices {
            for armor_choice in &armor_choices {
                for ring_choice in &ring_choices {
                    res.push(
                        weapon_choice
                            .iter()
                            .chain(armor_choice)
                            .chain(ring_choice)
                            .copied()
                            .copied()
                            .collect(),
                    );
                }
            }
        }
        res
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum CharacterKind {
    Player,
    Boss,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Character {
    kind: CharacterKind,
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    fn player(items: &[Item]) -> Self {
        let mut res = Character {
            kind: CharacterKind::Player,
            hp: 100,
            damage: 0,
            armor: 0,
        };
        for item in items {
            res.damage += item.damage;
            res.armor += item.armor;
        }
        res
    }

    fn boss(input: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let mut stats = re.find_iter(input).map(|m| m.as_str().parse().unwrap());
        Self {
            kind: CharacterKind::Boss,
            hp: stats.next().unwrap(),
            damage: stats.next().unwrap(),
            armor: stats.next().unwrap(),
        }
    }
}

fn fight(mut attacker: Character, mut defender: Character) -> Character {
    while attacker.hp > 0 {
        defender.hp -= cmp::max(1, attacker.damage - defender.armor);
        mem::swap(&mut attacker, &mut defender);
    }
    defender
}

pub fn part1(input: &str) -> i32 {
    let boss = Character::boss(input);
    Item::loadouts()
        .iter()
        .filter(|loadout| {
            fight(Character::player(loadout), boss.clone()).kind == CharacterKind::Player
        })
        .map(|loadout| loadout.iter().map(|i| i.cost).sum())
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> i32 {
    let boss = Character::boss(input);
    Item::loadouts()
        .iter()
        .filter(|loadout| {
            fight(Character::player(loadout), boss.clone()).kind == CharacterKind::Boss
        })
        .map(|loadout| loadout.iter().map(|i| i.cost).sum())
        .max()
        .unwrap()
}

pub fn tests() {
    assert_eq!(
        fight(
            Character {
                kind: CharacterKind::Player,
                hp: 8,
                damage: 5,
                armor: 5,
            },
            Character {
                kind: CharacterKind::Boss,
                hp: 12,
                damage: 7,
                armor: 2
            }
        ),
        Character {
            kind: CharacterKind::Player,
            hp: 2,
            damage: 5,
            armor: 5,
        },
    );
}
