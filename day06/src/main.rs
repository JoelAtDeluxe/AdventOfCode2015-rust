fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read file");
    // part1(&contents);
    part2(&contents);
}

#[derive(Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

fn to_coords(s: &str) -> Coordinate {
    let parts: Result<Vec<u32>, _> = s.split(",").map(|x| x.parse::<u32>() ).collect();
    let v = parts.expect("couldn't parse coordinates");
    Coordinate {x: v[0], y: v[1]}
}

fn alter_grid<T>(grid: &mut Vec<Vec<T>>, c1: Coordinate, c2: Coordinate, action: fn(&T)->T) {    
    for i in c1.y..c2.y+1 {
        for j in c1.x..c2.x+1 {
            let i = i as usize;
            let j = j as usize;
            grid[i][j] = action(&grid[i][j]);
        }
    }
}

/////// Part 2 specific

fn part2(contents: &str) {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for line in contents.lines() {
        process_step_part2(&mut grid, &line);
    }

    let result = sum_grid(&grid);
    println!("Number of lights lit: {}", result);

}

fn sum_grid(grid: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for row in grid.iter() {
        for cell in row.iter() {
            sum += cell;
        }
    }
    return sum;
}

fn process_step_part2(grid: &mut Vec<Vec<u32>>, command: &str) {    
    let sentence: Vec<&str> = command.split(" ").collect();
    
    match sentence.as_slice() {
        ["turn", "off", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |x| if *x == 0 {0} else {x-1});
        },
        ["turn", "on", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |x| x + 1);
        },
        ["toggle", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |x| x + 2);
        },
        _ => {panic!("Unexpected command: {}", command)}
    };
}

/////// Part 1 Specific

fn part1(contents: &str) {
    let mut grid = vec![vec![false; 1000]; 1000];

    for line in contents.lines() {
        process_step(&mut grid, &line);
    }

    let result = count_ons(&grid);
    println!("Number of lights lit: {}", result);
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



fn process_step(grid: &mut Vec<Vec<bool>>, command: &str) {    
    let sentence: Vec<&str> = command.split(" ").collect();
    
    match sentence.as_slice() {
        ["turn", "off", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |_x: &bool| false);
        },
        ["turn", "on", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |_x: &bool| true);
        },
        ["toggle", start, "through", end] => {
            alter_grid(grid, to_coords(start), to_coords(end), |x: &bool| !x);
        },
        _ => {panic!("Unexpected command: {}", command)}
    };
}

