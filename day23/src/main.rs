use day23;

use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    let commands = day23::parse_program(&data);
    // initialize registers to 0
    let registers = commands
        .iter()
        .filter_map(|cmd| cmd.register) // map command to non-none registers
        .map(|key| (key, 0))
        .collect::<HashMap<char, i32>>();
    
    let mut part_2_registers = registers.clone();

    //part 1
    let result_registers = day23::run(&commands, registers);

    println!("Register B after running the program: {}", result_registers.get(&'b').unwrap());

    // part 2
    part_2_registers.insert('a', 1);
    let result_registers = day23::run(&commands, part_2_registers);
    println!("Register B after re-running the program: {}", result_registers.get(&'b').unwrap());

}
