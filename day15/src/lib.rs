pub mod ingredient;
pub use ingredient::Ingredient;

mod recipe;
pub use recipe::Recipe;

mod score;
pub use score::Score;

pub fn as_int(really_an_int: &str) -> i32 {
    really_an_int.parse::<i32>().unwrap()
}

pub fn parse_input(content: &str) -> Vec<Ingredient> {
    let mut ingredients = Vec::new();

    for line in content.lines() {
        // sanitize line -- we don't need the pretty stuff
        let fixed_line = line.replace(':', "").replace(',', "");
        let parts: Vec<&str> = fixed_line.split(" ").collect();
        match parts.as_slice() {
            [name, "capacity", cap, "durability", dur, "flavor", flav, "texture", tex, "calories", cals] =>
            {
                let ing = Ingredient::new(
                    name,
                    as_int(cap),
                    as_int(dur),
                    as_int(flav),
                    as_int(tex),
                    as_int(cals),
                );
                ingredients.push(ing);
            }
            _ => panic!("Can't parse line"),
        }
    }
    ingredients
}

fn get_best_ingredient(ingredient_types: &Vec<Ingredient>, eval: fn(&Ingredient) -> i32) -> &Ingredient {
    let mut ing_iter = ingredient_types.iter();
    let mut best = ing_iter.next().unwrap();
    let mut best_score = eval(best);

    for ing in ing_iter {
        let new_score = eval(ing);
        if new_score > best_score {
            best = ing;
            best_score = new_score;
        }
    }

    best
}

// Doesn't work
pub fn build_optimal_recipe(base: &mut Recipe, ingredient_types: &Vec<Ingredient>, max_ingredients: i32) {
    let best_cap_ing = get_best_ingredient(ingredient_types, |ing| ing.capacity );
    let best_dur_ing = get_best_ingredient(ingredient_types, |ing| ing.durability );
    let best_flav_ing = get_best_ingredient(ingredient_types, |ing| ing.flavor );
    let best_text_ing = get_best_ingredient(ingredient_types, |ing| ing.texture );
    
    for _ in base.total_ingredients() .. max_ingredients {
        let cur_score = base.get_score();
        let selected_ingredient = match cur_score {
            _ if cur_score.capacity < 0 => best_cap_ing,
            _ if cur_score.durability < 0 => best_dur_ing,
            _ if cur_score.flavor < 0 => best_flav_ing,
            _ if cur_score.texture <= 0 => best_text_ing,
            _ => {
                let mut ing_iter = ingredient_types.iter();
                let mut best_addition = ing_iter.next().unwrap();
                let mut best_score = base.test_change(best_addition);
                for ing in ing_iter {
                    let new_score = base.test_change(ing);
                    if new_score > best_score {
                        best_addition = ing;
                        best_score = new_score;
                    }
                }
                best_addition
                // println!("Score after picking a new item: {:?}", base.get_score());
            }
        };
        base.add_ingredient(&selected_ingredient);
        let new_score = base.get_score();
        println!("Score after picking a new item: {:?} (picked: {} ;; total => {})", &new_score, selected_ingredient.name, new_score.total());
    }
}

pub fn brute_force_recipe_for_4_types(ingredient_types: &Vec<Ingredient>, max_ingredients: i32, is_valid: fn(&Recipe)->bool) -> Recipe {
    let mut best_recipe = Recipe::new(); // dummy placeholder -- will get replaced via loop

    let mut best_recipe_score = -1;
    for i in 1..max_ingredients {
        for j in 1..(max_ingredients - i) {
            for k in 1..(max_ingredients - i - j) {
                let remainder = max_ingredients - i - j - k;
                let quantities = vec![i, j, k, remainder];
                let junk: Vec<(&Ingredient, i32)> = ingredient_types
                    .iter()
                    .zip(quantities.into_iter())
                    .collect();

                let test_formula = Recipe::from_ingredients(&junk);
                if is_valid(&test_formula) {
                    let test_formula_score = test_formula.get_score().total();
                    if test_formula_score > best_recipe_score {
                        best_recipe = test_formula;
                        best_recipe_score = test_formula_score;
                    }
                }
            }
        }
    }
    best_recipe
}