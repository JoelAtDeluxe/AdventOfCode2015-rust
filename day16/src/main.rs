use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    let sues = parse_input(&data);

    // provided:
    // children: 3
    // cats: 7
    // samoyeds: 2
    // pomeranians: 3
    // akitas: 0
    // vizslas: 0
    // goldfish: 5
    // trees: 3
    // cars: 2
    // perfumes: 1

    let possible_sues: Vec<&Sue> = sues.iter()
        .filter(|s| loose_match(&s.attributes, "children", |x| x == 3))
        .filter(|s| loose_match(&s.attributes, "samoyeds", |x| x == 2))
        .filter(|s| loose_match(&s.attributes, "akitas", |x| x == 0))
        .filter(|s| loose_match(&s.attributes, "vizslas", |x| x == 0))
        .filter(|s| loose_match(&s.attributes, "cars", |x| x == 2))
        .filter(|s| loose_match(&s.attributes, "perfumes", |x| x == 1))
        .collect();

    let part_one_data = possible_sues;

    let possible_sues: Vec<&&Sue> = part_one_data.iter()
        .filter(|s| loose_match(&s.attributes, "cats", |x| x == 7))
        .filter(|s| loose_match(&s.attributes, "pomeranians", |x| x == 3))
        .filter(|s| loose_match(&s.attributes, "goldfish", |x| x == 5))
        .filter(|s| loose_match(&s.attributes, "trees", |x| x == 3))
        .collect();

    println!("Sue that sent the present: {}", possible_sues[0].number);

    let possible_sues: Vec<&&Sue> = part_one_data.iter()
        .filter(|s| loose_match(&s.attributes, "cats", |x| x > 7))
        .filter(|s| loose_match(&s.attributes, "trees", |x| x >3))
        .filter(|s| loose_match(&s.attributes, "pomeranians", |x| x < 3))
        .filter(|s| loose_match(&s.attributes, "goldfish", |x| x < 5))
        .collect();

    println!("Err, actually, the real sue is: {}", possible_sues[0].number)

}

fn loose_match(sue_atts: &HashMap<String, i32>, key: &str, eval: fn(i32)-> bool) -> bool {
    match sue_atts.get(key) {
        Some(x) => eval(*x),
        None => true
    }
}

struct Sue {
    number: i32,
    attributes: HashMap<String, i32>
}

impl Sue {
    fn new(name: i32) -> Sue {
        Sue{number: name, attributes: HashMap::new()}
    }

    fn with_attribute(&mut self, name: &str, value: &str) {
        // should probably check for errors TODO
        let val = value.parse::<i32>().unwrap();
        self.attributes.insert(String::from(name), val);
    }
}

fn parse_input(content: &str) -> Vec<Sue> {
    let mut sues = Vec::with_capacity(500);

    for line in content.replace(':', "").replace(',', "").lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        match parts.as_slice() { // each Sue seems to have 3 attributes -- if not, probably easier to parse with a regex
            ["Sue", number, att1, val1, att2, val2, att3, val3] => {
                let mut sue = Sue::new(number.parse::<i32>().unwrap());
                sue.with_attribute(att1, val1);
                sue.with_attribute(att2, val2);
                sue.with_attribute(att3, val3);

                sues.push(sue);
            }
            _ => panic!("Sue does not match expectations!")
        }
    }

    sues
}