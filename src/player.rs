

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
    
}

