use day15;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

//     let data = String::from(r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#);

    let ingredients = day15::parse_input(&data);

    // guess at solution -- did not work
    // let mut recipe = day15::Recipe::new();
    // for ing in ingredients.iter() {
    //     recipe.add_ingredient(ing);
    // }

    // day15::build_optimal_recipe(&mut recipe, &ingredients, 100);

    let recipe = day15::brute_force_recipe_for_4_types(&ingredients, 100, |r| true);

    println!("Best recipe score is: {}", recipe.get_score().total());
    // println!("Recipe: {:#?}", recipe);
    let recipe = day15::brute_force_recipe_for_4_types(&ingredients, 100, |r| r.get_calories() == 500);
    println!("Best recipe score at 500 cals: {}", recipe.get_score().total());

}

