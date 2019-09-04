
fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let mut containers: Vec<i32> = parse_input(&data);
    let target = 150;

    let mut all_combos = Vec::new();

    let num_combos = get_number_of_combos(containers.len());
    for i in 1..num_combos {
        let combo = get_combination(&containers, i);
        let sum:i32 = combo.iter().sum();
        
        if sum == target {
            all_combos.push(combo);
        }
    }
    
    //part 1
    println!("Number of container sets that can hold {} liters: {}", target, all_combos.len());

    // part 2
    let min_containers = all_combos.iter().map(|combo| combo.len()).min().unwrap();
    let min_container_set:Vec<&Vec<i32>> = all_combos.iter().filter(|combo| combo.len() == min_containers).collect();

    println!("Number of minimal container sets that can hold {} liters: {}", target, min_container_set.len());

}


fn parse_input(contents: &str) -> Vec<i32> {
    contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn get_number_of_combos(v_len: usize) -> i32 {
    (1 << v_len) as i32
}

fn get_combination(v: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut n = n;
    let mut subset = Vec::with_capacity(v.len());
    let mut index = 0;
    while n > 0 {
        let include = n & 0x1;
        n = n >> 1;
        if include == 1 {
            subset.push(v[index].clone());
        }
        index += 1;
    }
    subset
}