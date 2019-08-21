use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    let input = std::fs::read_to_string(path).expect(&format!("Failed to open: {}", path));

    

    // part 1 / "year 1"
    let mut year_one_homes = HashMap::new();
    let simple_itr = make_string_itr(0, 0, &input);
    add_gifs_to_home(&mut year_one_homes, simple_itr);
    println!("Year 1: Delivered gifts to {} homes", year_one_homes.len());

    // part 2 / "year 2"
    // let mut year_two_homes = HashMap::new();
    // let santa_list = input[::1];
    // let robo_list = input[::1];

    // println!("Starting postion is: {:?}", pos) 
}

fn make_string_itr(start: i32, skip: i32, content: &str) -> fn() -> Option<char> {
    let content_itr = content.chars();
    content_itr.skip(start as usize);
    let lambda = || {let rtn = content_itr.next(); content_itr.skip(skip as usize); rtn};

    lambda
    // fn itr() -> Option<char> {
    //     return content_itr.next();
    // }

    // itr
}

fn add_gifs_to_home(homes: &mut HashMap<(i32, i32), i32>, next_func: fn()->Option<char>) {
    let mut pos = (0, 0);
    let start = homes.entry(pos).or_default();
    *start += 1;

    loop {
        let ch = next_func();
        match ch {
            Some(c) => match c {
                    '>' => pos.1 += 1,
                    '<' => pos.1 -= 1,
                    'v' => pos.0 += 1,
                    '^' => pos.0 -= 1,
                    _ => ()
                },
            None => break
        };
        let entry = homes.entry(pos).or_default();
        *entry += 1;
    }

}