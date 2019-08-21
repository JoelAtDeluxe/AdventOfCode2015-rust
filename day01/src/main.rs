fn main() {
    let path = "input.txt";
    let input = std::fs::read_to_string(path).expect(&format!("Failed to open: {}", path));
    let (last_floor, basement_step) = analyze_input(&input);

    println!("With input: {}", &input[..]);
    println!("Last floor reached: {}", last_floor);
    match basement_step {
        Some(val) => println!("Step on which basement is reached (first time): {}", val),
        None => println!("Never reached the basement"),
    }
}

// not really needed unless doing some parsing
// fn read_file(path: &str) -> String {
//     let result = std::fs::read_to_string(path);
//     if result.is_err() {
//         panic!("Error trying to read file: {} ;; err text: {:?}", path, result.unwrap_err());
//     }
//     let contents = result.unwrap();
//     contents
// }

fn analyze_input(s: &str) -> (i32, Option<usize>) {
    let mut count = 0;
    let mut basement_step: Option<usize> = None;

    for (i, byte) in s.chars().enumerate() {
        match byte {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count < 0 && basement_step == None {
                    basement_step = Some(i + 1);
                }
            }
            _ => (),
        };
    }
    (count, basement_step)
}

// fn count_floors(s: &str) -> i32 {
//     let mut count = 0;
//     for byte in s.chars() {
//         match byte {
//             '(' => count += 1,
//             ')' => count -= 1,
//             _ => (),
//         };
//     }
//     count
// }

// fn first_basement(s: &str) -> Option<usize> {
//     let mut count = 0;
//     for (i, byte) in s.chars().enumerate() {
//         match byte {
//             '(' => count += 1,
//             ')' => {
//                 count -= 1;
//                 println!("step: {} => count: {}", i + 1, count);
//                 if count < 0 {
//                     return Some(i + 1);
//                 }
//             }
//             _ => (),
//         }
//     }
//     return None;
// }
