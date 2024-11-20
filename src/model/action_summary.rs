use crate::model::Item;
use std::fmt;

pub enum EscapeSummary {
    Success,
    Failure(u32),
}

impl fmt::Display for EscapeSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EscapeSummary::Success => {
                write!(f, "You successfully fled into the next room.")
            }
            EscapeSummary::Failure(damage_taken) => {
                write!(f, "You tried to flee into the next room unsuccessfully and took {} damage from your enemy!", damage_taken)
            }
        }
    }
}

pub struct BattleSummary {
    pub inflicted_damage: u32,
    pub taken_damage: u32,
    pub player_roll: u8,
    pub enemy_roll: u8,
}

impl fmt::Display for BattleSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You and your enemy engage in an exchange of hits. You rolled {} and dealt your enemy {} damage. Your enemy rolled {}, and dealt you {} damage.",
            self.player_roll, self.inflicted_damage, self.enemy_roll,
            self.taken_damage)
    }
}

pub struct PurchaseSummary {
    pub item_bought: Item,
}

impl fmt::Display for PurchaseSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You successfully bought {}!", self.item_bought.name())
    }
}

pub struct SaleSummary {
    pub item_sold: Item,
}

impl fmt::Display for SaleSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Successfully sold {}.", self.item_sold.name())
    }
}

pub struct EquippingSummary {
    pub item_equipped: Item,
}

impl fmt::Display for EquippingSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Successfully equipped {}.", self.item_equipped.name())
    }
}

pub enum ActionSummary {
    Fleed(EscapeSummary),
    Battled(BattleSummary),
    Bought(PurchaseSummary),
    Sold(SaleSummary),
    Equipped(EquippingSummary),
}

impl fmt::Display for ActionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ActionSummary::Battled(battle_summary) => write!(f, "{}", battle_summary),
            ActionSummary::Fleed(escape_summary) => write!(f, "{}", escape_summary),
            ActionSummary::Bought(purchase_summary) => write!(f, "{}", purchase_summary),
            ActionSummary::Sold(sale_summary) => write!(f, "{}", sale_summary),
            ActionSummary::Equipped(equipping_summary) => write!(f, "{}", equipping_summary),
        }
    }
}
