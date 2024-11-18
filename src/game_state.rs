use crate::action_summary::{
    ActionSummary, BattleSummary, EquippingSummary, EscapeSummary, PurchaseSummary, SaleSummary,
};
use crate::encounter::Encounter;
use crate::enemy::Enemy;
use crate::game::{Command, CommandExecutionError};
use crate::player::Player;
use std::fmt;

#[derive(Debug)]
pub struct GameState {
    room_number: u32,
    current_room: Encounter,
    player: Player,
}

impl GameState {
    /// Produces initial game state
    pub fn new() -> Self {
        Self {
            room_number: 0,
            current_room: Encounter::EmptyRoom,
            player: Player::new(),
        }
    }

    fn next_room(&mut self) {
        self.room_number += 1;
        self.current_room = Encounter::new_random();
    }

    /// Moves game state to next encounter if allowed
    fn flee(&mut self) -> Result<EscapeSummary, CommandExecutionError> {
        let flee_action_result = if self.current_room.can_trivially_escape() {
            EscapeSummary::Success
        } else {
            let player_roll = rand::random::<u8>() % 20 + 1;
            if self.current_room.can_non_trivially_escape(player_roll) {
                EscapeSummary::Success
            } else {
                let failed_flee_damage = self.current_room.calculate_failed_flee_damage();
                self.player.take_damage(failed_flee_damage);
                EscapeSummary::Failure(failed_flee_damage)
            }
        };
        match flee_action_result {
            EscapeSummary::Success => self.next_room(),
            EscapeSummary::Failure(_) => (),
        }
        Ok(flee_action_result)
    }

    fn battle_round(player: &mut Player, enemy: &mut Enemy) -> BattleSummary {
        let player_roll = player.roll_for_attack();
        let inflicted_damage = if enemy.is_hit_by(player_roll) {
            let damage = player.get_damage();
            enemy.take_damage(damage);
            damage
        } else {
            0
        };
        let enemy_roll = enemy.roll_for_attack();
        let taken_damage = if !enemy.is_dead() && player.is_hit_by(enemy_roll) {
            let damage = enemy.get_damage();
            player.take_damage(damage);
            damage
        } else {
            0
        };
        BattleSummary {
            player_roll,
            enemy_roll,
            inflicted_damage,
            taken_damage,
        }
    }

    /// Makes player battle opponent
    fn battle(&mut self) -> Result<BattleSummary, CommandExecutionError> {
        let potential_enemy = self.current_room.get_enemy();
        match potential_enemy {
            None => Err(CommandExecutionError::EnemyNotFound),
            Some(enemy) => Ok(GameState::battle_round(&mut self.player, enemy)),
        }
    }

    /// Buys item if shop
    fn buy(&mut self, item_name: &str) -> Result<PurchaseSummary, CommandExecutionError> {
        let shop_listing = self.current_room.find_price_listing(item_name);
        match shop_listing {
            Some(shop_listing) => {
                let bought = self.player.buy(shop_listing);
                if bought {
                    Ok(PurchaseSummary {
                        item_bought: shop_listing.item.clone(),
                    })
                } else {
                    Err(CommandExecutionError::InsufficientFunds)
                }
            }
            None => Err(CommandExecutionError::ItemNotFound),
        }
    }

    fn sell(&mut self, _item_name: &str) -> Result<SaleSummary, CommandExecutionError> {
        todo!()
    }

    fn equip(&mut self, _item_name: &str) -> Result<EquippingSummary, CommandExecutionError> {
        todo!()
    }

    /// Produces true if game is over
    pub fn game_over(&self) -> bool {
        self.player.is_dead()
    }

    pub fn get_shop_items(&self) -> Vec<&str> {
        self.current_room.get_shop_items()
    }

    pub fn get_inventory(&self) -> Vec<&str> {
        self.player.get_inventory()
    }

    pub fn execute_command(
        &mut self,
        command: Command,
    ) -> Result<ActionSummary, CommandExecutionError> {
        match command {
            Command::Flee => Ok(ActionSummary::Fleed(self.flee()?)),
            Command::Battle => Ok(ActionSummary::Battled(self.battle()?)),
            Command::Buy(item_name) => Ok(ActionSummary::Bought(self.buy(&item_name)?)),
            Command::Sell(item_name) => Ok(ActionSummary::Sold(self.sell(&item_name)?)),
            Command::Equip(item_name) => Ok(ActionSummary::Equipped(self.equip(&item_name)?)),
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Room number: {}. {}",
            self.room_number, self.current_room
        )
    }
}
