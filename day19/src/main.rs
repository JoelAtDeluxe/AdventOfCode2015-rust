use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let (rules, formula) = parse_input(&data);

    let num_combos = calibrate(&formula, &rules);

    println!("Calibrating...");
    println!("After Calibration, discovered {} combinations", num_combos);

    let elements = to_elements(&formula);
    let result = calc_minimum_steps_required(elements);

    println!("{}", result);
}

struct Replacement {
    given: String,
    output: String,
}

fn to_elements(formula: &str) -> Vec<String> {
    let mut rtn = Vec::new();

    let mut next_element = "";

    for c in formula.chars() {
        if c.is_ascii_lowercase() {
            let last_element_index = rtn.len() - 1;
            rtn[last_element_index] = format!("{}{}", rtn[last_element_index], c);
        } else {
            rtn.push(format!("{}", c));
        }
    }

    rtn
}

fn calc_minimum_steps_required(elements: Vec<String>) -> usize {
    // each replacement takes one of the following forms:
    // 1. M -> N.N (i.e. one element to two)
    // 2. M -> N.Rn.N.Ar (i.e. one element to two, with encapsulation)
    // 3. M -> N.Rn.N.Y.(N.Y.)N.Ar (i.e. Element, encapsulation with 1-2 elements)
    // If we asbstract this to consider total molecule length:
    // Rule 1 increases the length of the total molecule by 1
    // Rule 2 increases the length of the total molecule by 3
    // Rule 3 increases the length of the total molecule by 5-7
    // Note that for rule 3, it is actually the same as rule 2, but with the added rule that each Y encountered is 2 (additional) elements
    // knowing this, if we can deduce the number of elements we have, and sum each category,
    // we should be able to answer the question (if not find the formulation)

    let mut rtn = elements.len() - 1; // first character is free

    let y_count = elements.iter().filter(|i| **i == String::from("Y")).count();
    let ar_count = elements.iter().filter(|i| **i == String::from("Ar")).count();

    rtn = rtn - (2 * y_count) - (2 * ar_count);
    rtn
}

fn brute_force_part_2(formula: &str, rules: &Vec<Replacement>) {
    let mut next_set = HashSet::new();
    let mut current_set = HashSet::new();
    // let mut all_formulas = HashSet::new();
    // all_formulas.insert(formula.clone());
    next_set.insert(formula);
    let mut iterations = 0;
    let mut found = false;

    loop {
        println!("num formulas: {}", next_set.len());
        iterations += 1;
        current_set = next_set;
        next_set = HashSet::new();

        if current_set.is_empty() {
            break;
        }

        for item in current_set {
            let reduced = reduce(&item, &rules);
            for next_item in reduced {
                if next_item == "e" {
                    found = true;
                    break;
                }
                // if !all_formulas.contains(&next_item) {
                //     all_formulas.insert(next_item.clone());
                //     next_set.insert(next_item);
                // }
            }
        }
    }
    if found {
        println!("Number of iterations to make medicine: {}", iterations);
    } else {
        println!("No solution found. :(");
    }
}

fn reduce(formula: &str, rules: &Vec<Replacement>) -> Vec<String> {
    let mut variants = Vec::new();

    for rule in rules.iter() {
        let replacement = formula.replace(&rule.output, &rule.given);
        if replacement != formula {
            variants.push(replacement);
        }
    }

    variants
}

fn find_all_possible_replacements(
    calibration_string: &str,
    rules: &Vec<Replacement>,
) -> HashSet<String> {
    let mut combos = HashSet::new();

    for rule in rules.iter() {
        let parts: Vec<&str> = calibration_string.split(&rule.given).collect();
        for i in 0..parts.len() - 1 {
            let gaps: Vec<&String> = (0..parts.len() - 1)
                .map(|gap_idx| {
                    if gap_idx == i {
                        &rule.output
                    } else {
                        &rule.given
                    }
                })
                .collect();
            let mut recombined = Vec::with_capacity(parts.len() + gaps.len());
            for i in 0..gaps.len() {
                // recombined[2*i] = parts[i];
                // recombined[(2*i) + 1] = gaps[i];
                recombined.push(parts[i]);
                recombined.push(gaps[i]);
            }
            recombined.push(parts[parts.len() - 1]);
            let recombined = recombined.join("");
            combos.insert(recombined);
        }
    }

    combos
}

fn calibrate(calibration_string: &str, rules: &Vec<Replacement>) -> usize {
    let combos = find_all_possible_replacements(calibration_string, rules);

    combos.len()
}

fn parse_input(data: &str) -> (Vec<Replacement>, String) {
    let all_lines: Vec<&str> = data.lines().collect();
    let mut rules = Vec::new();

    // last line has the calibration string
    let cal_string = String::from(all_lines[all_lines.len() - 1]);

    for line in all_lines[..all_lines.len() - 1].iter() {
        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() == 2 {
            rules.push(Replacement {
                given: String::from(parts[0]),
                output: String::from(parts[1]),
            });
        }
    }

    (rules, cal_string)
}
