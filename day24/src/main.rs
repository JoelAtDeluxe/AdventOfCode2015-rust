
fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    let packages = parse_input(&data);
    let target_weight = packages.iter().sum::<u32>() / 3;

    // // test data
    // let packages = vec![5, 5, 10, 15, 20];
    // let target_weight = 25;

    // test data 2
    // let packages = vec![1,2,3,4,5, 7,8,9,10,11];
    // let target_weight = 20;
    
    // part 1

    let mut all_combos = Vec::with_capacity(3_000_000);
    let num_combos = get_number_of_combos(packages.len());
    for i in 1..num_combos {
        let combo = get_combination(&packages, i);
        
        if combo.iter().sum::<u32>() == target_weight {
            let alt_group = make_sub_set(&packages, &combo);
            let num_alt_combos = get_number_of_combos(alt_group.len());
            for j in 1..num_alt_combos {
                let alt_combo = get_combination(&alt_group, j);
                if alt_combo.iter().sum::<u32>() == target_weight {
                    all_combos.push(combo);
                    break;
                }
            }
        }
    }

    // println!("{:?}", all_combos);

    let ideal_size = all_combos.iter()
        .map(|v| v.len())
        .min().unwrap();
    let lowest_qe = all_combos.iter()
        .filter(|v| v.len() == ideal_size)
        .map(find_quantum_entanglement)
        .min().unwrap();

    println!("Lowest QE is: {}", lowest_qe);

    // println!("{:?}", all_combos)
}

fn parse_input(contents: &str) -> Vec<u32> {
    contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn make_sub_set(nums: &Vec<u32>, exclude: &Vec<u32>) -> Vec<u32> {
    let mut rtn = Vec::with_capacity(nums.len() - exclude.len());

    for v in nums.iter() {
        if !exclude.contains(v) {
            rtn.push(v.clone())
        }
    }
    rtn
}

fn get_number_of_combos(v_len: usize) -> u32 {
    (1 << v_len) as u32
}

fn get_combination(v: &Vec<u32>, n: u32) -> Vec<u32> {
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

// fn get_combination(v: &Vec<u32>, n: u32) -> Vec<&u32> {
//     let mut n = n;
//     let mut subset = Vec::with_capacity(v.len());
//     let mut index = 0;
//     while n > 0 {
//         let include = n & 0x1;
//         n = n >> 1;
//         if include == 1 {
//             subset.push(&v[index]);
//         }
//         index += 1;
//     }
//     subset
// }

fn find_quantum_entanglement(nums: &Vec<u32>) -> u32 {
    nums.iter().fold(1, |acc, item| acc * item)
}
