use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    let input = std::fs::read_to_string(path).expect(&format!("Failed to open: {}", path));

    // part 1 / "year 1"
    let mut year_one_homes = HashMap::new();
    add_gifs_to_home(&mut year_one_homes, &input);
    println!("Year 1: Delivered gifts to {} homes", year_one_homes.len());

    // part 2 / "year 2"
    let mut year_two_homes: HashMap<(i32, i32), i32> = HashMap::new();
    let mut santa_script = String::new();
    let mut robo_script = String::new();

    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => santa_script.push(c),
            1 => robo_script.push(c),
            _ => ()
        };
    }

    add_gifs_to_home(&mut year_two_homes, &santa_script);
    add_gifs_to_home(&mut year_two_homes, &robo_script);
    

    println!("Year 2: Delivered gifts to {} homes", year_two_homes.len());
}

fn add_gifs_to_home(homes: &mut HashMap<(i32, i32), i32>, script: &str) {
    let mut pos = (0, 0);
    let start = homes.entry(pos).or_default();
    *start += 1;

    for ch in script.chars() {
        match ch {
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            'v' => pos.0 += 1,
            '^' => pos.0 -= 1,
            _ => ()
        };
        let entry = homes.entry(pos).or_default();
        *entry += 1;
    }
}