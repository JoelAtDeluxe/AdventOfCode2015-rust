
use std::collections::HashMap;

use crate::ingredient::Ingredient;

#[path = "score.rs"]
mod score;
use score::Score;

#[derive(Clone, Debug)]
pub struct Recipe {
    used_ingredients: HashMap<String, (Ingredient, i32)>,
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe{used_ingredients: HashMap::new()}
    }
    pub fn from_ingredients(ingredients: &Vec<(&Ingredient, i32)>) -> Recipe {
        let mut recipe = Recipe::new();
        for (ing, qty) in ingredients {
            recipe.used_ingredients.insert(ing.name.clone(), ( (*ing).clone(), qty.clone()));
        }
        recipe
    }

    pub fn test_change(&self, ingredient: &Ingredient) -> i32 {
        let mut clone = self.clone();
        clone.add_ingredient(ingredient);
        clone.get_score().total()
    }

    pub fn add_ingredient(&mut self, ingredient: &Ingredient) {
        let mut entry = self.used_ingredients
            .entry(ingredient.name.clone())
            .or_insert((ingredient.clone(), 0));

        (*entry).1 += 1;
    }

    pub fn get_score(&self) -> Score {
        let mut score = Score::new();

        for (ing, qty) in self.used_ingredients.values() {
            score.capacity += ing.capacity * qty;
            score.durability += ing.durability * qty;
            score.flavor += ing.flavor * qty;
            score.texture += ing.texture * qty;
        }

        score
    }

    pub fn total_ingredients(&self) -> i32 {
        self.used_ingredients.values().map(|i| i.1).sum()
    }

    pub fn get_calories(&self) -> i32 {
        self.used_ingredients.values().map(|i| i.0.calories * i.1).sum()
    }
}