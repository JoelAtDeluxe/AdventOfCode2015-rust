fn main() {
    // not bothering to parse the input
    let solution_row = 3010;
    let solution_column = 3019;

    let solution = grid_number_to_code(find_grid_number(solution_row, solution_column));
    println!("Could should be: {}", solution);
}

fn find_grid_number(row: u64, col: u64) -> u64 {
    // 1-based numbers here
    let r1_cn = ((col * (col + 1)) / 2);

    let mut gap = 0;
    if row > 2 {
        gap = ((row - 2) * (row - 1)) / 2; // sum of all numbers, i = 1 : row - 2
    }
    let rn_cn = r1_cn + gap + ((row - 1) * col); // sum of numbers is actually offset, starting at row-1 (so, we would count: row-1, row, row+1, row+2, etc)
    rn_cn
}

fn grid_number_to_code(grid_num: u64) -> u64 {
    let base = 20151125; 
    let numerator_factor = 252533;
    let denominator =    33554393;

    let pow = grid_num - 1;
    let mut actual_factor = 1;
    for _ in 0..pow {
        actual_factor = (actual_factor * numerator_factor) % denominator;
    }

    (base * actual_factor) % denominator
}
