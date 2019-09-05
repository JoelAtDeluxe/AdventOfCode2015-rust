use day24;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let do_part_2 = false;

    let packages = parse_input(&data);
    let num_compartments = if do_part_2 {4} else {3};
    let target = packages.iter().sum::<u32>() / num_compartments;

    //look for the minimal set by testing those combinations first
    let mut ideal_set = Vec::new();
    for i in 1..=packages.len() { //we can slim this down quite a bit
        print!("Looking at combinations of size: {}", i);
        flush_stdout();
        let combo_numbers = day24::n_choose_m_combo_generator(packages.len(), i);
        println!(" (there are {} of these)", combo_numbers.len());
        let correct_sums:Vec<(usize, u32)> = combo_numbers
            .iter()
            .map(|combo| (*combo, sum_combination(&packages, *combo)) )
            .filter(|(_, sum)| *sum == target)
            .collect();
        
        if correct_sums.len() > 0 {
            println!("Ideal size is: {}", i);
            ideal_set = correct_sums;
            break;
        }
    }

    // augment with quantum entanglement
    println!("\tCalculating Entanglement...");
    let mut ideal_set: Vec<(usize, u32, u64)> = ideal_set
        .into_iter()
        .map(|(num, sum)| (num, sum, quantum_entanglement_for_combination(&packages, num) ))
        .collect();

    println!("\tSorting ideal set...");
    ideal_set.sort_by(|a, b| a.2.cmp(&b.2));

    println!("\tVerifying results...");
    let mask = (1 << packages.len()) - 1;
    let min_size = day24::count_ones(ideal_set[0].0);
    for (num, _, qe) in ideal_set.iter() {
        // look at the opposite set for num
        let opposite_set = mask & !num;
        let subset = day24::make_subset(&packages, opposite_set);
        let mut found = false;

        // check if some combination = target
        
        for subset_size in min_size .. subset.len() {
            let possible_combos = day24::n_choose_m_combo_generator(subset.len(), subset_size);
            let findings = possible_combos
                .iter()
                .map(|combo| sum_combination(&subset, *combo))
                .find(|sum| *sum == target);
            
            // if so, stop -- this is the lowest qe for the lowest set.
            if let Some(_) = findings {
                println!("The minimum Quantum Entanglement is: {}", qe);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

}

fn flush_stdout() {
    use std::io::prelude::*;
    use std::io;

    io::stdout().flush().ok().expect("Could not flush stdout");
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
