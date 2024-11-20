use rand::{self, Rng};

#[derive(Debug)]
pub struct Enemy {
    health: u32,
    damage: u32,
    armor_class: u8,
}

impl Enemy {
    pub fn new_goblin() -> Enemy {
        Enemy {
            health: 5,
            damage: 2,
            armor_class: 7,
        }
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = if self.health > damage {
            self.health - damage
        } else {
            0
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn roll_for_attack(&self) -> u8 {
        rand::thread_rng().gen_range(1..=20)
    }

    pub fn is_hit_by(&self, hit_roll: u8) -> bool {
        hit_roll > self.armor_class
    }
}
