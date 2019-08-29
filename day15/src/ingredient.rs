#[derive(Debug, Clone, Eq)]
pub struct Ingredient {
    pub name: String,
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

impl Ingredient {
    pub fn new(
        name: &str,
        capacity: i32,
        durability: i32,
        flavor: i32,
        texture: i32,
        calories: i32,
    ) -> Ingredient {
        Ingredient {
            name: String::from(name),
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

impl PartialEq for Ingredient {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}