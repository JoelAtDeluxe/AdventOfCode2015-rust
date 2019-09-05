use day24;
use day24::ComboStatus;

use std::cmp::Ordering;


fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    let packages = parse_input(&data);
    let target = packages.iter().sum::<u32>() / 3;

    // NOTE! This solution assumes that there are no 0 weight packages

    let num_combos = day24::get_number_of_combos(packages.len()) as usize;

    let mut checked_values = vec![ComboStatus::Unknown; num_combos];
    for i in 1 .. num_combos {
        if checked_values[i] == ComboStatus::Unknown {
            check_combo_v2(&packages, i, target, &mut checked_values);
        }
    }

    let valid_combos: Vec<Vec<u32>> = checked_values
        .into_iter()
        .enumerate()
        .filter(|(_, status)| *status == ComboStatus::KnownGood)
        .map(|(i, _) | day24::combo_to_subset(&packages, i) )
        .collect();
    
    let ideal_size = valid_combos
        .iter()
        .map(|item| item.len())
        .min().unwrap();
    
    // let ideal_size_set = valid_combos
    //     .iter()
    //     .map(|item| item.len() == ideal_size)
    //     .collect();

    let min_qe_for_ideal_size = valid_combos
        .iter()
        .filter(|item| item.len() == ideal_size)
        .map(|item| find_quantum_entanglement(item))
        .min().unwrap();

    // part 1:
    println!("Minimum QE from ideal set: {}", min_qe_for_ideal_size);
}

fn parse_input(contents: &str) -> Vec<u32> {
    contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn sum_digits(i: usize) -> usize {
    let mut sum = 0;
    day24::binary_iter(i, &mut |_|{ sum += 1;});
    sum 
}

fn check_combo_v2(v: &Vec<u32>, combo_number: usize, target: u32, checked_values: &mut Vec<ComboStatus>) {
    let combo_sum = sum_combination(v, combo_number);
    // this function takes advantage of the fact that sum(all(v)) = 3 * target
    // that is, that target + 2 * traget = sum(all(v))

    if sum_digits(combo_number) > 9 {
        return;
    }

    match combo_sum.cmp(&target) {
        Ordering::Less => {
            checked_values[combo_sum as usize] = ComboStatus::KnownBad;
            day24::mark_combo_proper_subsets_invalid(&day24::mk_index_map(v.len(), combo_number), checked_values);
            //opposite set will also be bad
        },
        Ordering::Greater => checked_values[combo_sum as usize] = ComboStatus::KnownBad,
        Ordering::Equal => {

            //step 0: we know proper subsets won't work, likewise, we know the opposite set is too big, so exclude those for the future
            day24::mark_combo_proper_subsets_invalid(&day24::mk_index_map(v.len(), combo_number), checked_values);

            let opposite = day24::mask_num(v.len(), !combo_number); // since combo_number is the target, opposite must be 2*target
            checked_values[opposite] = ComboStatus::KnownBad;
            
            //step 1: find all subsets of the opposite set that sum to the target
            let mut some_alternate_subset_exists = false;

            let opposite_index_map = day24::mk_index_map(v.len(), opposite);
            let num_opposite_combos = day24::get_number_of_combos(opposite_index_map.len()) as usize;

            for i in 1 .. num_opposite_combos {
                let mapped_opposite_combo = day24::mk_combo_from_subcombo(&opposite_index_map, i) as usize;

                match checked_values[mapped_opposite_combo] {
                    ComboStatus::KnownBad => continue,
                    ComboStatus::KnownGood => { some_alternate_subset_exists = true; continue; }
                    _ => {}
                }

                let opposite_combo_sum = sum_combination(&v, mapped_opposite_combo);
                let opposite_trimmed_index_map = day24::mk_trimmed_index_map(&opposite_index_map, i);

                //let flipped = day24::mask_num(opposite_index_map.len(), i ^ opposite); // all subset bits flipped -- we use this in all match arms
                let mapped_flipped_combo = day24::mask_num(v.len(), mapped_opposite_combo ^ opposite);

                let flipped_trimmed_index_map = day24::mk_index_map(v.len(), mapped_flipped_combo);

                //step 2a: If a subset in the opposite set is valid, mark it valid, otherwise invalid
                //step 2b: If a subset is valid in step 2a, then it's opposite is valid. If it's not, then it's opposite is also not
                // NOTE: At this point, we know that the remainder of the values sum to 2*target
                match opposite_combo_sum.cmp(&target) {
                    Ordering::Equal => {
                        some_alternate_subset_exists = true;
                        checked_values[mapped_flipped_combo] = ComboStatus::KnownGood;
                        // subsets are no-good
                        day24::mark_combo_proper_subsets_invalid(&opposite_trimmed_index_map, checked_values);
                        day24::mark_combo_proper_subsets_invalid(&flipped_trimmed_index_map, checked_values);
                    }
                    Ordering::Less => {
                        checked_values[mapped_opposite_combo] = ComboStatus::KnownBad;
                        checked_values[mapped_flipped_combo] = ComboStatus::KnownBad;
                        day24::mark_combo_proper_subsets_invalid(&opposite_trimmed_index_map, checked_values);
                    }
                    Ordering::Greater => {
                        checked_values[mapped_opposite_combo] = ComboStatus::KnownBad;
                        checked_values[mapped_flipped_combo] = ComboStatus::KnownBad;
                        day24::mark_combo_proper_subsets_invalid(&flipped_trimmed_index_map, checked_values);
                    },
                }
            }
            
            //step 3: If any subset in the opposite set is valid, mark combo_number as valid
            checked_values[combo_number] = if some_alternate_subset_exists {
                ComboStatus::KnownGood
            } 
            else {
                ComboStatus::KnownBad
            };
        }
    }
}

fn check_combo_v2_copy(v: &Vec<u32>, combo_number: usize, target: u32, checked_values: &mut Vec<ComboStatus>) {
    let combo_sum = sum_combination(v, combo_number);

    match combo_sum.cmp(&target) {
        Ordering::Less => {
            checked_values[combo_sum as usize] = ComboStatus::KnownBad;
            day24::mark_combo_proper_subsets_invalid(&day24::mk_index_map(v.len(), combo_number), checked_values);
        },
        Ordering::Greater => checked_values[combo_sum as usize] = ComboStatus::KnownBad,
        Ordering::Equal => {
            // Note: Possible recursion tactic here
            // could be good, could be bad...
            
            //step 1: find all subsets of the opposite set that sum to the target
            //step 2a: If a subset in the opposite set is valid, mark it valid, otherwise invalid
            //step 2b: If a subset is valid in step 2a, then it's opposite is valid. If it's not, then it's opposite is also not
            //step 3: If any subset in the opposite set is valid, mark combo_number as valid
        }
    }
}


fn sum_combination(v: &Vec<u32>, combo_number: usize) -> u32 {
    let mut sum = 0;
    let mut n = combo_number;
    let mut index = 0;
    
    while n > 0 {
        if (n & 0x1) != 0 {
            sum += v[index];
        }
        n = n >> 1;
        index += 1;
    }
    sum
}

fn find_quantum_entanglement(nums: &Vec<u32>) -> u32 {
    nums.iter().fold(1, |acc, item| acc * item)
}
