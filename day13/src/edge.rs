use std::cmp::{Ord, Ordering};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub person_a: String,
    pub person_b: String,
    pub weight: i32
}

impl Edge {
    pub fn new(person_a: String, person_b: String, weight: i32) -> Edge {
        Edge{person_a, person_b, weight}
    }

    pub fn print(&self) { // should maybe be replaced by display?
        println!("{} + {} = {}", self.person_a, self.person_b, self.weight);
    }

    pub fn other_name(&self, name_a: &str) -> String {
        if name_a == self.person_a {
            return String::from(&self.person_b);
        }
        return String::from(&self.person_a);
    }

    pub fn nodes(&self) -> (String, String) {
        (self.person_a.clone(), self.person_b.clone())
    }
}
