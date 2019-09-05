use day24;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let do_part_2 = true;

    let packages = parse_input(&data);
    let target = if do_part_2 {
        packages.iter().sum::<u32>() / 4
    }
    else {
        packages.iter().sum::<u32>() / 3
    };

    //look for the minimal set by testing those combinations first
    let mut ideal_set = Vec::new();
    for i in 1..=packages.len() { //we can slim this down quite a bit
        println!("Looking at combinations of size: {}", i);
        let combo_numbers = day24::n_choose_m_combo_generator(packages.len(), i);
        println!("\t\t (there are {} many of these)", combo_numbers.len());
        let correct_sums:Vec<(usize, u32)> = combo_numbers
            .iter()
            .map(|combo| (*combo, sum_combination(&packages, *combo)) )
            .filter(|(_, sum)| *sum == target)
            .collect();
        
        if correct_sums.len() > 0 {
            println!("Found correct sums using combinations of size: {}", i);
            ideal_set = correct_sums;
            break;
        }
    }

    // augment with quantum entanglement
    let mut ideal_set: Vec<(usize, u32, u64)> = ideal_set
        .into_iter()
        .map(|(num, sum)| (num, sum, quantum_entanglement_for_combination(&packages, num) ))
        .collect();

    ideal_set.sort_by(|a, b| a.2.cmp(&b.2));
    println!("min is: {}", ideal_set[0].2);

    // for (num, sum, qe) in ideal_set.iter() { // TODO
    //     // look at the opposite set for num
    //     // check if some combination = target
    //     // if so, stop -- this is the lowest qe for the lowest set.
    // }

}

fn parse_input(contents: &str) -> Vec<u32> {
    contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn sum_combination(v: &Vec<u32>, combo_number: usize) -> u32 {
    let mut sum = 0;

    day24::binary_iter(combo_number, &mut |index| {
        sum += v[*index];
    });

    sum
}

fn quantum_entanglement_for_combination(v: &Vec<u32>, combo_number: usize) -> u64 {
    let mut qe = 1u64;

    day24::binary_iter(combo_number, &mut |index| {
        qe = qe * v[*index] as u64;
    });

    qe
}
