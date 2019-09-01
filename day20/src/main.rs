use day20;

fn main() {
    // let deliveries = simulate_gift_delivery(51);
    // day20::print_house_deliveries(&deliveries, 10);
    // println!("----------------");
    // let deliveries = simulate_gift_delivery_part_2(51);
    // day20::print_house_deliveries(&deliveries, 10);


    // Part 1 solution
    let target = 36_000_000;
    let first_house = day20::part_one_brute_force(target);
    println!("First house with {} presents: {}", target, first_house);
    // part 2 solution
    let second_house = day20::part_two_brute_force(target, first_house);
    println!("For lazy elves, first house with {} presents: {}", target, second_house);


    // let magic = day20::get_presents_for_house_number_gp(51, 10, |f, h| f * 50 >= *h);
    // println!("Guess: {}", magic);
}

fn simulate_gift_delivery_gp(num_houses: usize, should_stop: fn(usize, usize)->bool) -> Vec<u32>{
    let mut houses: Vec<u32> = vec![0; num_houses]; // every house gets a delivery from the first elf!
    let num_elves = num_houses; // just for ease in understanding

    for elf in 0..num_elves {
        let elf_index = elf + 1;
        for i in 0..num_houses {
            let house_index = i + 1;
            if house_index % elf_index == 0 {
                houses[i] += elf_index as u32;
                if should_stop(elf_index, house_index) {
                    break
                }
            }
        }
    }

    houses
}

fn simulate_gift_delivery(num_houses: usize) -> Vec<u32> {
    simulate_gift_delivery_gp(num_houses, |_, _| false)
}

fn simulate_gift_delivery_part_2(num_houses: usize) -> Vec<u32> {
    simulate_gift_delivery_gp(num_houses, |elf, house| {
        house >= 50 * elf
    })
}


