use std::collections::{HashMap, HashSet};

use regex::Regex;

fn parse(input: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    Regex::new(r"(.+) \(contains (.+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            (
                captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split(' ')
                    .collect(),
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split(", ")
                    .collect(),
            )
        })
        .collect()
}

fn allergens_to_ingredients<'a>(
    foods: &[(HashSet<&'a str>, HashSet<&'a str>)],
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut res: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, allergens) in foods {
        for &allergen in allergens {
            if let Some(dangerous) = res.get_mut(allergen) {
                *dangerous = dangerous.intersection(ingredients).copied().collect();
            } else {
                res.insert(allergen, ingredients.clone());
            }
        }
    }
    res
}

pub fn part1(input: &str) -> usize {
    let foods = parse(input);

    let dangerous: HashSet<&str> = allergens_to_ingredients(&foods)
        .values()
        .flatten()
        .copied()
        .collect();

    foods
        .into_iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|&ingredient| !dangerous.contains(ingredient))
        .count()
}

pub fn part2(input: &str) -> String {
    let mut allergens_to_ingredients = allergens_to_ingredients(&parse(input));

    let mut ingredients_to_allergens = HashMap::new();
    while let Some((&allergen, ingredients)) = allergens_to_ingredients
        .iter()
        .find(|(_, ingredients)| ingredients.len() == 1)
    {
        let ingredient = *ingredients.iter().next().unwrap();
        ingredients_to_allergens.insert(ingredient, allergen);
        allergens_to_ingredients.remove(allergen);
        for ingredients in allergens_to_ingredients.values_mut() {
            ingredients.remove(&ingredient);
        }
    }

    let mut ingredients: Vec<&str> = ingredients_to_allergens.keys().copied().collect();
    ingredients.sort_unstable_by_key(|ingredient| ingredients_to_allergens[ingredient]);
    ingredients.join(",")
}

pub fn tests() {
    let example = "
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    ";
    assert_eq!(part1(example), 5);
    assert_eq!(part2(example), "mxmxvkd,sqjhc,fvjkl");
}
