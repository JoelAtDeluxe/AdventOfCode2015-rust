use std::cmp;

fn main() {
    let gifts = parse_input("input.txt");

    let total_paper_needed: u32 = (&gifts).into_iter()
        .map(|gift| gift.paper_required())
        .sum();

    println!("Total amount of paper needed: {}", total_paper_needed);
    
    let total_ribbon_needed: u32 = gifts.into_iter()
        .map(|gift| gift.ribbon_required())
        .sum();

    println!("Total amount of ribbon needed: {}", total_ribbon_needed);

    //this should result in, probably, 1586300
}

struct Gift {
    length: u32,
    width: u32,
    height: u32
}

impl Gift {
    pub fn new (s: &str) -> Result<Gift, String>{
        let parts: Vec<&str> = s.split("x").collect();

        match parts.len() {
            3 => {
                let result: Result<Vec<u32>, _> = parts.into_iter().map(|x| x.parse::<u32>() ).collect();
                match result {
                    Ok(lwh) => Ok(Gift {length: lwh[0], width: lwh[1], height: lwh[2]}),
                    Err(e) => Err( format!("Could not parse values. Error: {}", e)),
                }
            },
            _ => Err( format!("Input didn't match expected form: {}", s)),
        }        
    }

    pub fn paper_required(&self) -> u32 {
        let surface_area = 2 * ((self.length * self.height) + (self.width * self.height) + (self.length * self.width));
        let min_area = (self.length * self.width * self.height) / cmp::max(self.height, cmp::max(self.length, self.width));
        surface_area + min_area
    }

    pub fn ribbon_required(&self) -> u32 {
        self.volume() + self.shortest_perimeter()
    }

    fn shortest_perimeter(&self) -> u32 {
        let total = self.length + self.width + self.height;
        let smallest_half_parimeter = total - cmp::max(self.length, cmp::max(self.width, self.height));
        
        smallest_half_parimeter * 2
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

fn parse_input(path: &str) -> Vec<Gift> {
    let input = std::fs::read_to_string(path).expect("Failed to open: input.txt");
    let mut gifts = Vec::new();

    for line in input.lines() {
        let gift = Gift::new(line);
        gifts.push(gift.unwrap_or_else(|err| panic!("Could not construct a gift: {}", err) ));
    }
    gifts
}