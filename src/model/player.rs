use crate::model::ShopListing;
use crate::model::{Armor, Item, Weapon};
use rand::{self, Rng};

#[derive(Debug)]
pub struct Player {
    health: u32,
    money: u32,
    equipped_armor: Armor,
    equipped_weapon: Weapon,
    inventory: Vec<Item>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 5,
            money: 2,
            equipped_armor: Armor::new_adventurer_cap(),
            equipped_weapon: Weapon::new_dagger(),
            inventory: Vec::new(),
        }
    }

    // Roll for damage
    pub fn roll_for_attack(&self) -> u8 {
        rand::thread_rng().gen_range(1..=20)
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = if self.health > damage {
            self.health - damage
        } else {
            0
        };
    }

    pub fn get_damage(&self) -> u32 {
        self.equipped_weapon.damage
    }

    pub fn is_hit_by(&self, hit_roll: u8) -> bool {
        hit_roll > self.equipped_armor.armor_class
    }

    pub fn buy(&mut self, listing: &ShopListing) -> bool {
        if self.money >= listing.price {
            self.money -= listing.price;
            self.inventory.push(listing.item.clone());
            true
        } else {
            false
        }
    }

    // Checks if player is dead
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn get_inventory(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for item in self.inventory.iter() {
            out.push(item.name());
        }
        out
    }
}
