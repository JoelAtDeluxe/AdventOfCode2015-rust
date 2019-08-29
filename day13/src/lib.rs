pub mod edge;
pub use edge::Edge;

pub mod arrangement;
pub use arrangement::Arrangement;

pub fn parse_input(file_data: &str) -> Vec<Edge> {
    let mut edges = Vec::new();

    for line in file_data.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        match parts.as_slice() {
            [person_a, "would", gain_lose, x, "happiness", "units", "by", "sitting", "next", "to", person_b] => {
                let factor = if *gain_lose == "gain" {1} else {-1};
                let points = x.parse::<i32>().unwrap() * factor;
                let sanitized_person_a = String::from(*person_a);
                let sanitized_person_b = remove_dot(person_b);
                edges.push(Edge::new(sanitized_person_a, sanitized_person_b, points));
            },
            _ => panic!("Input not in the right format!")
        }
    }

    edges
}

pub fn merge_edges(edges: Vec<Edge>) -> Vec<Edge> {
    let mut rtn = Vec::new();
    let mut calculated_edges = Vec::new();

    for edge in edges.iter() {
        if !calculated_edges.contains(&edge) {
            let opposite_edge = edges.iter()
                .filter(|e| e.person_b == edge.person_a && edge.person_b == e.person_a)
                .collect::<Vec<&Edge>>()[0];
            calculated_edges.push(opposite_edge);

            rtn.push(Edge::new(edge.person_a.clone(), edge.person_b.clone(), edge.weight + opposite_edge.weight));
        }
    }

    rtn
}

pub fn remove_dot(name: &str) -> String {
    if name.ends_with(".") {
        return String::from(&name[..name.len() - 1]);
    }
    String::from(name)
}

pub fn print_all_edges(edges: &Vec<Edge>) {
    for edge in edges.iter() {
        edge.print();
    }
}

pub fn create_arrangements(existing_arrangement: &Arrangement, all_edges: &Vec<Edge>) -> Vec<Arrangement> {
    let mut arrangements = Vec::new();
    let target = existing_arrangement.right_name();

    let valid_edges: Vec<&Edge> = all_edges.iter()
        .filter(|edge|  // all edges that have our last person, but don't have anyone else we've used before
            (edge.person_a == target && !existing_arrangement.has_name(&edge.person_b)) || 
            (edge.person_b == target && !existing_arrangement.has_name(&edge.person_a)) )
        .collect();

    if valid_edges.len() == 0 {
        //need to factor in seat 0 + seat n score
        let left = existing_arrangement.left_name();
        let right = existing_arrangement.right_name();

        let missing_edge = all_edges.iter().filter(|edge| 
            (edge.person_a == left && edge.person_b == right) ||
            (edge.person_b == left && edge.person_a == right)
            ).collect::<Vec<&Edge>>()[0];
        let mut copy = existing_arrangement.clone();
        copy.score += missing_edge.weight;
        arrangements.push(copy);
    }
    else {
        for edge in valid_edges.iter() {
            let mut cloned = existing_arrangement.clone();
            cloned.score += edge.weight;
            cloned.add_name(&edge.other_name(&target));
            let mut sub_arrangement = create_arrangements(&mut cloned, all_edges);
            arrangements.append(&mut sub_arrangement);
        }
    }
    
    arrangements
}