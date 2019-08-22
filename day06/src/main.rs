fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read file");
    let mut grid = vec![vec![false; 1000]; 1000];

    for line in contents.lines() {
        process_step(&mut grid, &line);
    }

    let result = count_ons(&grid);
    println!("Number of lights lit: {}", result);
}

#[derive(Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

fn count_ons(grid: &Vec<Vec<bool>>) -> u32 {
    let mut count = 0;

    for row in grid.iter() {
        for cell in row.iter() {
            if *cell {
                count+= 1;
            }
        }
    }
    return count;
}

fn to_coords(s: &str) -> Coordinate {
    let parts: Result<Vec<u32>, _> = s.split(",").map(|x| x.parse::<u32>() ).collect();
    let v = parts.expect("couldn't parse coordinates");
    Coordinate {x: v[0], y: v[1]}
}

fn process_step(grid: &mut Vec<Vec<bool>>, command: &str) {    
    let sentence: Vec<&str> = command.split(" ").collect();
    
    match sentence.as_slice() {
        ["turn", "off", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |_x: bool| false);
        },
        ["turn", "on", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |_x: bool| true);
        },
        ["toggle", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |x: bool| !x);
        },
        _ => {panic!("Unexpected command: {}", command)}
    };
}

fn alter_grid(grid: &mut Vec<Vec<bool>>, c1: Coordinate, c2: Coordinate, action: fn(bool)->bool) {    
    for i in c1.y..c2.y+1 {
        for j in c1.x..c2.x+1 {
            let i_index = i as usize;
            let j_index = j as usize;
            grid[i_index][j_index] = action(grid[i_index][j_index]);
        }
    }
}