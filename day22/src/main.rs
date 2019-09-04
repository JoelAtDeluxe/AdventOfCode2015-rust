fn main() {
    // not bothering with parsing

    let player = Character::new("You", 50, 500, 0, 0, vec![
        Ability::new("Magic Missile", 53, None, 4, 0),
        Ability::new("Drain",         73, None, 2, 2),
        Ability::new("Shield",       113, Some(Effect::new(EffectType::Shielded,   6, TargetType::PartyMember)), 0, 0),
        Ability::new("Poison",       173, Some(Effect::new(EffectType::Poisoned,   6, TargetType::Opponent)), 0, 0),
        Ability::new("Recharge",     229, Some(Effect::new(EffectType::Recharging, 5, TargetType::PartyMember)), 0, 0),
    ]);

    let boss = Character::new("Dragon Lord", 58, 0, 0, 9, Vec::new());

    battle(&player, &boss, false);
}

#[derive(Clone, Debug)]
enum TargetType {
    PartyMember,
    Opponent,
}

#[derive(Clone, Debug)]
enum EffectType {
    Shielded,
    Poisoned,
    Recharging,
}

#[derive(Clone, Debug)]
struct Effect {
    duration: i32,
    name_of_effect: EffectType,
    target: TargetType
}

impl Effect {
    fn new(name_of_effect: EffectType, duration: i32, target: TargetType) -> Effect {
        // Adding 1 to duration to reflect that we count down before we act on the effect
        Effect{name_of_effect, duration: duration + 1, target}
    }
}

#[derive(Clone, Debug)]
struct Character {
    name: String,
    hp: i32,
    mp: i32,
    ac: i32,
    dm: i32,
    skills: Vec<Ability>,
    effects: Vec<Effect>,
    ac_bonus: i32,
}

impl Character {
    fn new(name: &str, hp: i32, mp: i32, ac: i32, dm: i32, skills: Vec<Ability>) -> Character {
        Character{name: String::from(name), hp, mp, ac, dm, skills, effects: Vec::new(), ac_bonus: 0}
    }
    fn tick(&mut self) {
        self.ac_bonus = 0;
        for effect in self.effects.iter_mut() {
            effect.duration -= 1;
        }
        self.effects.retain(|e| e.duration > 0);

        for e in self.effects.iter() {
            match e.name_of_effect {
                EffectType::Shielded => self.ac_bonus = 7,
                EffectType::Poisoned => self.hp -= 3,
                EffectType::Recharging => self.mp += 101,
            }
        }
    }
    fn get_ar(&self) -> i32 {
        self.ac + self.ac_bonus
    }
    fn choose_action(&self) -> Option<Ability> {
        if self.skills.len() == 0 {
            return None
        }
        return None // TODO
    }
}

#[derive(Clone, Debug)]
struct Ability {
    name: String,
    mana_cost: i32,
    added_effect: Option<Effect>,
    instant_damage: i32,
    instant_healing: i32,
}

impl Ability {
    fn new(name: &str, 
           mana_cost: i32, added_effect: Option<Effect>, instant_damage: i32, instant_healing: i32) -> Ability {
        Ability{name: String::from(name), mana_cost, added_effect, instant_damage, instant_healing}
    }
}

fn battle(player: &Character, boss: &Character, narate: bool) {
    let narrator = |message: String| {
        if narate {
            println!("{}", message);
        }
    };

    let mut turn = 0;
    let mut player_clone = player.clone();
    let mut boss_clone = boss.clone();
    let mut actors = vec![&mut player_clone, &mut boss_clone];

    // narrator(format!("A {} draws near.", boss_clone.name));

    while actors[0].hp > 0 && actors[1].hp > 0 {
        narrator(format!("-- Turn {} --", turn + 1));
        // define roles for turn
            // these references won't work. =(
            // let mut actor = actors[turn % 2];
            // let mut opponent = actors[(turn + 1) % 2];
        let actor_idx = turn % 2;
        let opponent_idx = (turn+1) % 2;

        // check effects
        for actor in actors.iter_mut() {
            (*actor).tick();
        }

        // character acts
        let maybe_action: Option<Ability> = actors[actor_idx].choose_action();
        if let Some(action) = maybe_action {
            // narrator(format!("{} casts {}", player_clone.name, action.name));
            actors[opponent_idx].hp -= action.instant_damage;
            if action.instant_damage > 0 {
                narrator(format!("{}'s hitpoints have been reduced by {}", "???", action.instant_damage));
            }

            actors[actor_idx].hp += action.instant_healing;
            if action.instant_healing > 0 {
                narrator(format!("Thy hast recovered {} hitpoints", action.instant_healing));
            }

            let maybe_added_effect = action.added_effect;
            if let Some(effect) = maybe_added_effect {
                match effect.target {
                    TargetType::Opponent => actors[opponent_idx].effects.push(effect.clone()),
                    TargetType::PartyMember => actors[actor_idx].effects.push(effect.clone()),
                }
            }
        }
        else {
            let damage_done = actors[actor_idx].dm - actors[opponent_idx].get_ar();
            actors[opponent_idx].hp -= damage_done;
        }

        turn += 1;
    }
    // evaluate effects
    // character acts

}