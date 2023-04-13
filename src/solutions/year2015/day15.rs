use std::cmp::max;

use regex::Regex;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse(input: &str) -> Vec<Ingredient> {
    Regex::new(
        r"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap()
    .captures_iter(input)
    .map(|captures| Ingredient {
        capacity: captures[1].parse().unwrap(),
        durability: captures[2].parse().unwrap(),
        flavor: captures[3].parse().unwrap(),
        texture: captures[4].parse().unwrap(),
        calories: captures[5].parse().unwrap(),
    })
    .collect()
}

fn score(ingredients: &[Ingredient], recipe: &[i32]) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    for (ingredient, teaspoons) in ingredients.iter().zip(recipe) {
        capacity += teaspoons * ingredient.capacity;
        durability += teaspoons * ingredient.durability;
        flavor += teaspoons * ingredient.flavor;
        texture += teaspoons * ingredient.texture;
    }
    max(0, capacity) * max(0, durability) * max(0, flavor) * max(0, texture)
}

fn calories(ingredients: &[Ingredient], recipe: &[i32]) -> i32 {
    ingredients
        .iter()
        .zip(recipe)
        .map(|(ingredient, teaspoons)| teaspoons * ingredient.calories)
        .sum()
}

fn search<F: FnMut(&[i32])>(i: usize, teaspoons: i32, recipe: &mut [i32], handler: &mut F) {
    if i == recipe.len() - 1 {
        recipe[i] = teaspoons;
        handler(recipe);
    } else {
        for j in 0..=teaspoons {
            recipe[i] = j;
            search(i + 1, teaspoons - j, recipe, handler);
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let ingredients = parse(input);
    let mut recipe = vec![0; ingredients.len()];
    let mut max_score = 0;
    search(0, 100, &mut recipe, &mut |recipe| {
        max_score = max(score(&ingredients, recipe), max_score);
    });
    max_score
}

pub fn part2(input: &str) -> i32 {
    let ingredients = parse(input);
    let mut recipe = vec![0; ingredients.len()];
    let mut max_score = 0;
    search(0, 100, &mut recipe, &mut |recipe| {
        if calories(&ingredients, recipe) == 500 {
            max_score = max(score(&ingredients, recipe), max_score);
        }
    });
    max_score
}

pub fn tests() {
    let example = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                   Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(part1(example), 62_842_880);
    assert_eq!(part2(example), 57_600_000);
}
