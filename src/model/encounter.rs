use crate::model::Enemy;
use crate::model::{Armor, Item, Weapon};
use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct ShopListing {
    pub price: u32,
    pub item: Item,
}

#[derive(Debug)]
pub struct Shop {
    items: Vec<ShopListing>,
}

impl Shop {
    pub fn new() -> Shop {
        Shop {
            items: vec![
                ShopListing {
                    price: 1,
                    item: Item::Weapon(Weapon::new_dagger()),
                },
                ShopListing {
                    price: 2,
                    item: Item::Armor(Armor::new_adventurer_cap()),
                },
            ],
        }
    }

    pub fn find_item(&self, item_name: &str) -> Option<&ShopListing> {
        self.items
            .iter()
            .find(|listing: &&ShopListing| -> bool { listing.item.name() == item_name })
    }

    pub fn get_items(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for listing in &self.items {
            out.push(listing.item.name());
        }
        return out;
    }
}

#[derive(Debug)]
pub enum Encounter {
    EmptyRoom,
    Enemy(Enemy),
    Shop(Shop),
}

impl Encounter {
    /// Generates a new random encounter
    pub fn new_random() -> Self {
        let encounter_code = rand::thread_rng().gen_range(0..=2);
        match encounter_code {
            0 => Encounter::EmptyRoom,
            1 => Encounter::Enemy(Enemy::new_goblin()),
            2 => Encounter::Shop(Shop::new()),
            _ => unreachable!(),
        }
    }

    pub fn can_trivially_escape(&self) -> bool {
        match self {
            Encounter::EmptyRoom => true,
            Encounter::Enemy(enemy) => enemy.is_dead(),
            Encounter::Shop(_) => true,
        }
    }

    pub fn can_non_trivially_escape(&self, roll: u8) -> bool {
        match self {
            Encounter::EmptyRoom => true,
            Encounter::Enemy(_) => roll > 10,
            Encounter::Shop(_) => true,
        }
    }

    pub fn calculate_failed_flee_damage(&self) -> u32 {
        match self {
            Encounter::EmptyRoom => 0,
            Encounter::Enemy(enemy) => enemy.get_damage(),
            Encounter::Shop(_) => 0,
        }
    }

    pub fn get_enemy(&mut self) -> Option<&mut Enemy> {
        match self {
            Encounter::EmptyRoom => None,
            Encounter::Enemy(enemy) => {
                if !enemy.is_dead() {
                    Some(enemy)
                } else {
                    None
                }
            }
            Encounter::Shop(_) => None,
        }
    }

    pub fn find_price_listing(&mut self, item_name: &str) -> Option<&ShopListing> {
        match self {
            Encounter::EmptyRoom => None,
            Encounter::Enemy(_) => None,
            Encounter::Shop(shop) => shop.find_item(item_name),
        }
    }

    pub fn get_shop_items(&self) -> Vec<&str> {
        match self {
            Encounter::EmptyRoom => Vec::new(),
            Encounter::Enemy(_) => Vec::new(),
            Encounter::Shop(shop) => shop.get_items(),
        }
    }
}

impl fmt::Display for Encounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Encounter::EmptyRoom => write!(f, "You find yourself in an empty room."),
            Encounter::Enemy(_) => write!(f, "An enemy has been spotted!"),
            Encounter::Shop(_) => write!(f, "Welcome to the shop!"),
        }
    }
}
