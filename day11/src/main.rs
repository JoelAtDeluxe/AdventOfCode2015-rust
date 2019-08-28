use day11;

fn main() {
    let old_password = "vzbxkghb"; // guess: vzbxxyzz (that was right) ;; part 2 guess 2: vzccdeea (wrong!) ~vzccabcc~ vzccbcdd (wrong!)

    let mut possible = day11::advance_to_next_reasonable_password(old_password);

    while !is_valid_password(&possible) {
        possible = day11::advance_password(&possible);
    }
    println!("Valid password #1: {}", possible);
    possible = day11::advance_password(&possible);

    while !is_valid_password(&possible) {
        possible = day11::advance_password(&possible);
    }
    println!("Valid password #2: {}", possible);
}

fn is_valid_password(pass: &str) -> bool{
    day11::password_rule_three_in_order(pass) &&
    day11::password_rule_two_doubles(pass)
}


