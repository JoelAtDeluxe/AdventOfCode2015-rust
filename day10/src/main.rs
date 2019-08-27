fn main() {
    let input = "3113322113";
    let part_one_iters = 40;
    let part_two_iters = 50;

    // test input
    // let input = "1";
    // let part_one_iters = 5;
    // let part_two_iters = 6;
    
    let mut copy = String::from(input);
    for i in 0..part_one_iters { 
        copy = expand(&copy);
        println!("{} : ({})", (i+1), copy.len());
    }
    println!("^^ Part 1 solution");
    for i in part_one_iters..part_two_iters {
        copy = expand(&copy);
        println!("{} : ({})", (i+1), copy.len());
    }
    println!("^^ Part 2 solution");
}

fn expand(look: &str) -> String{
    let mut say = String::new();
    let mut num_matches = 0;
    let mut last_ch: Option<char> = None;

    for ch in look.chars() {
        match last_ch {
            None => {num_matches = 1; last_ch = Some(ch);},
            Some(c) if c == ch => num_matches += 1,
            Some(c) if c != ch => {
                say.push_str(&num_matches.to_string());
                say.push(last_ch.unwrap());
                num_matches = 1;
                last_ch = Some(ch);
            },
            Some(_) => unreachable!()
        }
    }
    say.push_str(&num_matches.to_string());
    say.push(last_ch.unwrap());

    say
}