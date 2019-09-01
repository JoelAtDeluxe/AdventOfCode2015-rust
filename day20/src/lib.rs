
pub fn print_house_deliveries(deliveries: &Vec<u32>, scale: u32) {
    for (idx, d) in deliveries.iter().enumerate() {
        println!("House {} got {} presents.", idx + 1, d * scale);
    }
}

pub fn factor(n: u32, sorted: bool) -> Vec<u32> {
    let mut factors = vec![1];
    let limit = 1 + (n as f64).sqrt() as u32;
    for i in 2..limit {
        if n % i == 0 {
            factors.push(i);
            if i as f64 != (n as f64 / i as f64) {
                factors.push(n / i);
            }
        }
    }

    factors.push(n);
    if sorted {
        factors.sort();
    }
    factors
}

pub fn part_one_brute_force(target: u32) -> u32 {
    // this can be castly improved by trying to get some bounds around it
    // you might be able to factor the number derive the answer from that?
    let mut sum = 10;
    let mut house = 1;
    while sum < target {
        house += 1;
        sum = get_presents_for_house_number(house);
    }
    house
}

pub fn part_two_brute_force(target: u32, start_at: u32) -> u32 {
    let mut sum = 0;
    let mut house = start_at - 1;

    while sum < target {
        house += 1;
        sum = get_presents_for_house_number_gp(house, 11, |f, h| f * 50 >= *h);
    }
    house
}

fn get_presents_for_house_number(house_number: u32) -> u32 {
    // let factors = factor(house_number, false);
    // let sum: u32 = factors.iter()
    //     .map(|i| *i as u32)
    //     .sum();
    // sum * 10
    get_presents_for_house_number_gp(house_number, 10, |_, _| true)
}

pub fn get_presents_for_house_number_gp(
    house_number: u32,
    gifs_per_elf: u32,
    filter_func: fn(&u32, &u32) -> bool,
) -> u32 {
    let factors = factor(house_number, false);
    let sum: u32 = factors
        .iter()
        .filter(|f| filter_func(*f, &house_number))
        .map(|i| *i as u32)
        .sum();
    sum * gifs_per_elf
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        assert_eq!(vec![1,2,3,6,17,34,51,102], factor(102, true) )
    }
}