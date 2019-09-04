use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let(rules, formula) = parse_input(&data);

    let num_combos = calibrate(&formula, &rules);

    println!("Calibrating...");
    println!("After Calibration, discovered {} combinations", num_combos);

    // solve(&formula, &rules);
}

struct Replacement {
    given: String,
    output: String
}

// fn solve(input: &str, rules: &Vec<Replacement>) {
//     let mut reduced_pattern = String::from(input);

//     let mut some_match_found = true; // asume there's at least one match
//     while some_match_found {
//         some_match_found = false;
//         for rule in rules {
//             if reduced_pattern.contains(&rule.output) {
//                 some_match_found = true;
//                 reduced_pattern = reduced_pattern.replace(&rule.output, &rule.given);
//                 break
//             }
//         }
//     }
//     println!("Final Pattern: {}", reduced_pattern);
    
// }

fn calibrate(calibration_string: &str, rules: &Vec<Replacement>) -> usize {
    let mut combos = HashSet::new();

    for rule in rules.iter() {
        let parts: Vec<&str> = calibration_string.split(&rule.given).collect();
        for i in 0..parts.len()-1 {
            let gaps:Vec<&String> = (0..parts.len()-1)
                .map(|gap_idx| if gap_idx == i { &rule.output } else { &rule.given})
                .collect();
            let mut recombined = Vec::with_capacity(parts.len() + gaps.len());
            for i in 0 .. gaps.len() {
                // recombined[2*i] = parts[i];
                // recombined[(2*i) + 1] = gaps[i];
                recombined.push(parts[i]);
                recombined.push(gaps[i]);
            }
            recombined.push(parts[parts.len()-1]);
            let recombined = recombined.join("");
            if rule.given == "Si" {
                println!("Combo: {}", recombined);
            }
            combos.insert(recombined);
        }
    }

    combos.len()
}

fn parse_input(data: &str) -> (Vec<Replacement>, String) {
    let all_lines: Vec<&str> = data.lines().collect();
    let mut rules = Vec::new();

    // last line has the calibration string
    let cal_string = String::from(all_lines[all_lines.len() - 1]);

    for line in all_lines[..all_lines.len()-1].iter() {
        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() == 2 {
            rules.push(Replacement{given: String::from(parts[0]), output: String::from(parts[1])});
        }
    }

    (rules, cal_string)
}