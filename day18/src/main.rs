

type Grid = Vec<Vec<i8>>;


fn parse_input(content: &str) -> Grid{
    let mut grid = Vec::with_capacity(100);

    for line in content.lines() { 
        let row = line.chars().map(|c| if c == '#' {1} else {0}).collect();
        grid.push(row);
    }

    grid
}

fn new_grid(size: usize) -> Grid {
    vec![vec![0; size]; size]
}

fn cell_value(grid: &Grid, r: i32, c: i32) -> i8 {
    if r < 0 || c < 0 {
        return 0;
    }
    if let Some(row) = grid.get(r as usize) {
        if let Some(cell) = row.get(c as usize) {
            return *cell
        }
    }
    0
}

fn broken_cell_value(grid: &Grid, r: i32, c: i32) -> i8 {
    if r < 0 || c < 0 {
        return 0;
    }
    if let Some(row) = grid.get(r as usize) {
        if let Some(cell) = row.get(c as usize) {
            return *cell
        }
    }
    0
}

fn sum_grid(grid: &Grid) -> u32 {
    grid.iter().flat_map(|r| r).map(|v| *v as u32).sum()
}

fn advance(grid: &Grid) -> Grid {
    let mut future_grid = new_grid(grid.len());

    let count_neighbors = |r, c| {
        cell_value(grid, r-1, c-1) + cell_value(grid, r-1, c) + cell_value(grid, r-1, c+1) + 
        cell_value(grid, r  , c-1) +          0               + cell_value(grid, r  , c+1) + 
        cell_value(grid, r+1, c-1) + cell_value(grid, r+1, c) + cell_value(grid, r+1, c+1)
    };

    let (on, off) = (1, 0);

    for row_idx in 0 .. grid.len() {
        for cell_idx in 0 .. grid[row_idx].len() {
            let fut_val = match (grid[row_idx][cell_idx], count_neighbors(row_idx as i32, cell_idx as i32)) {
                (1, 2) | (1, 3) => on,
                (1, _) => off,
                (0, 3) => on,
                (0, _) => off,
                _ => unreachable!()
            };
            future_grid[row_idx][cell_idx] = fut_val;
        }
    }

    future_grid
}

fn light_corners(grid: &mut Grid) {
    // assume square conifguration
    let last_cell = grid.len()-1;

    grid[0][0] = 1;
    grid[0][last_cell] = 1;
    grid[last_cell][0] = 1;
    grid[last_cell][last_cell] = 1;
}

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    let mut grid = parse_input(&data);
    let mut broken_grid = grid.clone();
    let num_steps = 100;

    for _ in 0 .. num_steps {
        grid = advance(&grid);
    }

    println!("Number of lights on after {} steps: {}", num_steps, sum_grid(&grid) );

    // initial lighting configuration may not have the corners lit, so doing this
    light_corners(&mut broken_grid);

    for _ in 0 .. num_steps {
        broken_grid = advance(&broken_grid);
        //set the 4 corners to on
        light_corners(&mut broken_grid);
    }

    println!("Number of lights on after {} steps (with broken lights):  {}", num_steps, sum_grid(&broken_grid))
}
