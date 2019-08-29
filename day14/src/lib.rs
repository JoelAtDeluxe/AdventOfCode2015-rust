

#[derive(Debug)]
pub struct Deer {
    pub name: String,
    rate: i32,
    sprint_duration: i32,
    sleep_duration: i32
}

impl Deer {
    pub fn new(name: &str, rate: i32, sprint_duration: i32, sleep_duration: i32) -> Deer {
        Deer{name: String::from(name), rate, sprint_duration, sleep_duration}
    }
}

fn as_int(really_an_int: &str) -> i32 {
    really_an_int.parse::<i32>().unwrap()
}

pub fn parse_input(contents: &str) -> Vec<Deer> {
    let mut deers = Vec::new();
    
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts.as_slice() {
            [name, "can", "fly", 
             rate, "km/s", "for", 
             sprint, "seconds,", "but", "then", "must", "rest", "for", 
             sleep, "seconds."] => {
                deers.push(Deer::new(name, as_int(rate), as_int(sprint), as_int(sleep)));
            },
            _ => panic!("Can't parse line"),
        }
    }

    deers
}

pub fn race(deer: &Deer, duration: i32) -> i32 {
    let cycle_length = deer.sleep_duration + deer.sprint_duration;
    let full_cycles = duration / cycle_length;
    let remainder = duration % cycle_length;

    let last_leg_secs = remainder.min(deer.sprint_duration);
    
    (full_cycles * deer.rate * deer.sprint_duration) + last_leg_secs * deer.rate
}

// pub fn proper_generator(sprint_duration: i32, sleep_duration: i32, rate: i32) {
//     let mut distance = 0;
//     let tick = || {
//         loop {
//             for i in 0..sprint_duration {
//                 distance += rate;
//                 yield distance;
//             }
//             for i in 0..sleep_duration {
//                 yield distance;
//             }
//         }
//     };

//     tick
// }

pub fn score_race(racers: &Vec<Deer>, duration: i32) -> Vec<i32> {
    //this would be WAY easier with a generator
    let mut scores = vec![0; racers.len()];
    
    let mut traveled_distances = vec![0; racers.len()];
    let cycle_distances: Vec<i32> = racers.iter()
        .map(|d| d.sprint_duration + d.sleep_duration)
        .collect();

    for tick in 0..duration {
        for (idx, deer) in racers.iter().enumerate() {

            let increase = if tick % cycle_distances[idx] < deer.sprint_duration {deer.rate} else {0};
            traveled_distances[idx] += increase;
        }
        let winners = calc_winners(&traveled_distances);
        for (idx, won) in winners.iter().enumerate() {
            if *won {
                scores[idx] += 1;
            }
        }
    }

    scores
}

fn calc_winners(vals: &Vec<i32>) -> Vec<bool> {
    let vals_itr = vals.iter();

    let mut max_idx: usize = 0;

    for (i, v) in vals_itr.enumerate() {
        if v > &vals[0] {
            max_idx = i;
        }
    }

    vals.iter().map(|v| *v == vals[max_idx]).collect::<Vec<bool>>()
}