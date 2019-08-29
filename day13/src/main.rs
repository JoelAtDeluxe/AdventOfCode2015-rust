use std::collections::HashSet;

use day13;
use day13::{Arrangement, Edge};

fn main() {
    let path = "input.txt";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));
    // let data = get_test_data();  // test data

    let all_edges = day13::parse_input(&data);
    let mut simplified_edges = day13::merge_edges(all_edges);
    let mut people_attending: HashSet<String> = simplified_edges
        .iter()
        .flat_map(|edge| vec![edge.person_a.clone(), edge.person_b.clone()])
        .collect();


    let all_arrangements = make_all_arrangements(&people_attending, &simplified_edges);
    let best_arrangement = get_best_arrangement(&all_arrangements);
    println!("Best arrangement: {:?}", best_arrangement);

    // Part 2 -- add self, dummy
    for person in people_attending.iter() {
        simplified_edges.push(Edge::new(String::from("self"), person.clone(), 0));
    }

    people_attending.insert(String::from("self"));
    let all_arrangements = make_all_arrangements(&people_attending, &simplified_edges);
    let best_arrangement = get_best_arrangement(&all_arrangements);
    println!("Best arrangement, including self: {:?}", best_arrangement);

    //for arr in all_arrangements {
    //     println!("{:?}", arr);
    // }
}

fn make_all_arrangements(peeps: &HashSet<String>, all_edges: &Vec<Edge>) -> Vec<Arrangement> {
    let mut all_arrangements = Vec::new();

    for person in peeps.iter() {
        let starting_point = Arrangement::new(&person);
        all_arrangements.append( &mut day13::create_arrangements(&starting_point, all_edges) );
    }

    all_arrangements
}

fn get_best_arrangement(all: &Vec<Arrangement>) -> &Arrangement {
    let mut best_arrangement = &all[0];
    for arr in all.iter() {
        if arr.score > best_arrangement.score {
            best_arrangement = &arr;
        }
    }
    best_arrangement
}

fn get_test_data() -> String {
    String::from(r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#)
}

