fn main() {
    println!("Hello, world!");
    // not going to bother with parsing the input for this one
    let boss = Character::new(109, 8, 2);
    let player = Character::new(100, 0, 0);
    let weapon_store = make_weapon_store();
    let armor_store = make_armor_store();
    let ring_store = make_ring_store();

    let (cheapest, most_expensive) = simulate(&player, &boss, &weapon_store, &armor_store, &ring_store);

    if let Some(min) = cheapest {
        println!("Cheapest win is: {}", min);
    }
    else {
        println!("Cannot win at any cost");
    }

    if let Some(max) = most_expensive {
        println!("Most Expensive loss is: {}", max);
    }
    else {
        println!("Cannot lose at any cost");
    }

    // println!("Will win: {}", simulate_fight(&player, &boss));
}

fn simulate(
    player: &Character,
    boss: &Character,
    weapon_store: &Store,
    armor_store: &Store,
    ring_store: &Store,
) -> (Option<i32>, Option<i32>) {
    let mut cheapest_win: Option<i32> = None;
    let mut most_expensive_loss: Option<i32> = None;
    let mut configuration = Vec::new();

    // all configurations
    for weapon_index in 0..weapon_store.len() {
        for armor_index in 0..armor_store.len() {
            for ring_1_idx in 0..ring_store.len() {
                for ring_2_idx in 0..ring_store.len() {
                    // Set the scenario
                    let weapon = &weapon_store[weapon_index];
                    let armor = &armor_store[armor_index];
                    let ring1 = &ring_store[ring_1_idx];
                    let ring2 = &ring_store[ring_2_idx];

                    // Can't buy the same ring... unless it's Nothing
                    if ring_1_idx == ring_2_idx {
                        continue;
                    }

                    let full_cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    let test_config = vec![weapon, armor, ring1, ring2];

                    let clone = Character::with_equipment(&player, &test_config);
                    let player_won = simulate_fight(&clone, &boss);

                    if player_won {
                        if cheapest_win.is_none() || cheapest_win.unwrap() > full_cost {
                            cheapest_win = Some(full_cost);
                            configuration = test_config;
                        }
                    }
                    else {
                        if most_expensive_loss.is_none() || most_expensive_loss.unwrap() < full_cost {
                            most_expensive_loss = Some(full_cost);
                        }
                    }
                }
            }
        }
    }

    println!("Cheapest config: {:?}", configuration);

    (cheapest_win, most_expensive_loss)
}

type Store = Vec<Item>;

#[derive(Debug)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
    purchased: bool,
}

impl Item {
    fn new(name: &str, cost: i32, damage: i32, armor: i32) -> Item {
        Item {
            name: String::from(name),
            cost,
            damage,
            armor,
            purchased: false,
        }
    }
}

#[derive(Clone, Debug)]
struct Character {
    base_hp: i32,
    hp: i32,
    dm: i32,
    ar: i32,
}

impl Character {
    fn new(hp: i32, dm: i32, ar: i32) -> Character {
        Character {
            base_hp: hp,
            hp,
            dm,
            ar,
        }
    }
    fn with_equipment(model: &Character, items: &Vec<&Item>) -> Character {
        let mut clone = model.clone();
        clone.equip_all(items);
        clone
    }
    fn struck(&mut self, opponent: &Character) {
        let damage_dealt = (opponent.dm - self.ar).max(1);
        self.hp -= damage_dealt;
    }
    fn is_alive(&self) -> bool {
        self.hp > 0
    }
    fn equip(&mut self, item: &Item) {
        self.dm += item.damage;
        self.ar += item.armor;
    }
    fn equip_all(&mut self, items: &Vec<&Item>) {
        for item in items.iter() {
            self.equip(item);
        }
    }
}

fn simulate_fight(player: &Character, boss: &Character) -> bool {
    let success;
    let damage_to_player = (boss.dm - player.ar).max(1);
    let damage_to_boss = (player.dm - boss.ar).max(1);
    let turns_to_kill_player = player.hp / damage_to_player + (player.hp % damage_to_player).min(1);
    let turns_to_kill_boss = boss.hp / damage_to_boss + (boss.hp % damage_to_boss).min(1);

    success = turns_to_kill_boss <= turns_to_kill_player;

    success
}

fn make_weapon_store() -> Vec<Item> {
    vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ]
}

fn make_armor_store() -> Vec<Item> {
    vec![
        Item::new("Nothing", 0, 0, 0),
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ]
}

fn make_ring_store() -> Vec<Item> {
    vec![
        Item::new("Nothing", 0, 0, 0),
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ]
}
