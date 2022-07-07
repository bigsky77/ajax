pub enum Class {
    Warrior,
    Mage,
    Rogue,
    Priest,
    Warlock,
    Shaman,
    Paladin,
    Hunter,
    Druid,
}

pub struct Player {
    pub name: String,
    pub class: Class,
    pub health: u32,
    pub damage: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "Ajax".to_string(),
            class: Class::Warrior,
            health: 100,
            damage: 10,
        }
    }
}

