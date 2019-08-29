use day14;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let time_to_travel = 2503;

    let deers = day14::parse_input(&data);

    let distances: Vec<i32> = deers.iter().map(|deer| day14::race(deer, time_to_travel)).collect();

    let pairings: Vec<(String, i32)> = deers.iter()
        .map(|d| d.name.clone())
        .zip(distances.into_iter())
        .collect();

    let mut pairing_itr = pairings.iter();
    let mut best_deer = pairing_itr.next().unwrap();
    for deer in pairing_itr {
        if deer.1 > best_deer.1 {
            best_deer = deer;
        }
    }
    
    println!("Maximum distance traveled: {} ({})", best_deer.1, best_deer.0);

    let scores = day14::score_race(&deers, time_to_travel);
    let pairings: Vec<(String, i32)> = deers.iter()
        .map(|d| d.name.clone())
        .zip(scores.into_iter())
        .collect();
    let mut pairing_itr = pairings.iter();
    let mut best_deer = pairing_itr.next().unwrap();
    for deer in pairing_itr {
        if deer.1 > best_deer.1 {
            best_deer = deer;
        }
    }

    println!("Maximum points scored: {} ({})", best_deer.1, best_deer.0);

}


