use day08;

fn main() {
    let path = "input.js";
    let input =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

// example scenario input 
//     let input =
// r#"""
// "abc"
// "aaa\"aaa"
// "\x27""#;

    let lines: Vec<&str> = input.lines().collect();
    let input_len = lines.iter().map(|l| l.len()).sum::<usize>() as i32;

    let memory_size: u16 = lines
        .iter()
        .map(|s| day08::analyze_line(s))
        .map(|x| x.0 - x.1)
        .sum();

    println!("Memory used by plain lines: {}", memory_size);

    // code to check what the lines and resulting sizes is (for debugging -- no longer needed)
    // let encoded_lines: Vec<String> = lines.iter().map(|s| day08::escape_line(s)).map(|s| format!("{} => {}", s, s.len())).collect();
    // println!("{:#?}", encoded_lines);


    let encoded_len: i32 = lines
        .iter()
        .map(|s| day08::escape_line(s))
        .map(|s| s.len() + 2)  // each line is 2 less than it should be (quotes not being counted properly, maybe?) 
        .sum::<usize>() as i32;

    println!(
        "Encoded lines length ({}) - input line length ({}): {}",
        encoded_len, input_len, encoded_len - input_len
    )
}
