#[derive(Debug)]
pub struct Score {
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

impl Score {
    pub fn new() -> Score {
        Score{
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
    pub fn total(&self) -> i32 {
        // self.capacity * self.durability * self.flavor * self.texture
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
}