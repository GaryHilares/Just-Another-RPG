#[derive(Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_class: u8,
}

impl Armor {
    pub fn new_adventurer_cap() -> Armor {
        Armor {
            name: String::from("Adventurer cap"),
            armor_class: 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub damage: u32,
}

impl Weapon {
    pub fn new_dagger() -> Weapon {
        Weapon {
            name: String::from("Dagger"),
            damage: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Armor(Armor),
    Weapon(Weapon),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Armor(Armor { name, .. }) | Item::Weapon(Weapon { name, .. }) => &name,
        }
    }
}
