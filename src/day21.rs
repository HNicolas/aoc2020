use std::collections::{HashMap, HashSet};

struct IngredientList<'a> {
    ingredients_count: HashMap<&'a str, u32>,
    possible_allergens: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> IngredientList<'a> {
    fn new(input: &'a str) -> Self {
        let (ingredients_count, possible_allergens) = input.lines().fold(
            (
                HashMap::<&str, u32>::new(),
                HashMap::<&str, HashSet<&str>>::new(),
            ),
            |(mut ingredients_count, mut possible_allergens), line| {
                let mut line_parts = line.split(" (contains ");
                let ingredients = line_parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .collect::<HashSet<_>>();
                ingredients.iter().for_each(|&ingredient| {
                    if let Some(count) = ingredients_count.get_mut(ingredient) {
                        *count += 1;
                    } else {
                        ingredients_count.insert(ingredient, 1);
                    }
                });
                let allergens = line_parts
                    .next()
                    .unwrap()
                    .trim_end_matches(')')
                    .split(", ")
                    .collect::<Vec<_>>();
                for allergen in allergens {
                    if let Some(possible_ingredients) = possible_allergens.get_mut(allergen) {
                        possible_ingredients.retain(|ingredient| ingredients.contains(ingredient))
                    } else {
                        possible_allergens.insert(allergen, ingredients.clone());
                    }
                }
                (ingredients_count, possible_allergens)
            },
        );
        Self {
            ingredients_count,
            possible_allergens,
        }
    }

    fn reduce_possible_allergens(&mut self) {
        let mut found_allergens: Vec<(&str, &str)> = self
            .possible_allergens
            .iter()
            .filter(|(allergen, ingredients)| ingredients.len() == 1)
            .map(|(allergen, ingredient)| (*allergen, *ingredient.iter().next().unwrap()))
            .collect::<Vec<_>>();
        while found_allergens.len() > 0 {
            let mut next = vec![];
            for (found_allergen, ingredient) in found_allergens {
                for (&allergen, ingredients) in self.possible_allergens.iter_mut() {
                    if allergen != found_allergen && ingredients.contains(ingredient) {
                        ingredients.remove(ingredient);
                        if ingredients.len() == 1 {
                            next.push((allergen, *ingredients.iter().next().unwrap()));
                        }
                    }
                }
            }
            found_allergens = next;
        }
    }
}

fn solve_1(ingredient_list: &IngredientList) -> u32 {
    let all_possible_allergens = ingredient_list
        .possible_allergens
        .values()
        .flatten()
        .collect::<HashSet<_>>();
    ingredient_list
        .ingredients_count
        .iter()
        .fold(0u32, |acc, (ingredient, count)| {
            if all_possible_allergens.contains(ingredient) {
                acc
            } else {
                acc + count
            }
        })
}

fn solve_2(list: &mut IngredientList) -> String {
    list.reduce_possible_allergens();
    let mut vec = list
        .possible_allergens
        .iter()
        .map(|(allergen, ingredients)| (*allergen, *ingredients.iter().next().unwrap()))
        .collect::<Vec<_>>();
    vec.sort_by(|(allergen, _), (allergen2, _)| allergen.cmp(allergen2));
    vec.iter()
        .map(|(_, ingredient)| *ingredient)
        .collect::<Vec<_>>()
        .join(",")
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day21").unwrap();
    let mut list = IngredientList::new(&input);
    println!(
        "day 21 solution 1 : {}, {}us",
        solve_1(&list),
        timer.elapsed().as_micros()
    );
    println!(
        "day 21 solution 2 : {}, {}us",
        solve_2(&mut list),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let list = IngredientList::new(&input);
        assert_eq!(solve_1(&list), 5);
    }

    #[test]
    fn test_solution_2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let mut list = IngredientList::new(&input);
        assert_eq!(solve_2(&mut list), "");
    }
}
