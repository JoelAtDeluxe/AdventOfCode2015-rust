
#[derive(Clone, Debug)]
pub struct Arrangement {
    pub ordering: Vec<String>,
    pub score: i32
}

impl Arrangement {
    pub fn new(head_of_table: &str) -> Arrangement {
        Arrangement{score: 0, ordering: vec![String::from(head_of_table)]}
    }

    pub fn left_name(&self) -> String {
        self.ordering[0].clone()
    }

    pub fn right_name(&self) -> String {
        self.ordering[ self.ordering.len() - 1].clone()
    }

    pub fn has_name(&self, name: &str) -> bool {
        self.ordering.contains(&String::from(name))
    }

    pub fn add_name(&mut self, new_name: &str) {
        self.ordering.push(String::from(new_name));
    }

    pub fn add_name_to_left(&mut self, new_name: &str) {
        self.ordering.insert(0, String::from(new_name));
    }

    pub fn edge_names(&self) -> (String, String) {
        (self.left_name(), self.right_name())
    }
}