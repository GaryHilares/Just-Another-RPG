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

pub struct PurchaseSummary<'a> {
    pub item_bought: &'a str,
}

impl<'a> fmt::Display for PurchaseSummary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You successfully bought {}!", self.item_bought)
    }
}

pub struct SaleSummary<'a> {
    pub item_sold: &'a str,
}

impl<'a> fmt::Display for SaleSummary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Successfully sold {}.", self.item_sold)
    }
}

pub struct EquippingSummary<'a> {
    pub item_equipped: &'a str,
}

impl<'a> fmt::Display for EquippingSummary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Successfully equipped {}.", self.item_equipped)
    }
}

pub enum ActionSummary<'a> {
    Fleed(EscapeSummary),
    Battled(BattleSummary),
    Bought(PurchaseSummary<'a>),
    Sold(SaleSummary<'a>),
    Equipped(EquippingSummary<'a>),
}

impl<'a> fmt::Display for ActionSummary<'a> {
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
