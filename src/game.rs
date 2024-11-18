use crate::game_state::GameState;
use inquire::Select;
use std::fmt;

pub enum Command {
    Flee,
    Battle,
    Buy(String),
    Sell(String),
    Equip(String),
}

pub enum CommandExecutionError {
    EnemyNotFound,
    ItemNotFound,
    InsufficientFunds,
}

enum CommandParsingError {
    ShouldRetry,
    ShouldExit,
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandExecutionError::EnemyNotFound => {
                write!(f, "You swing your sword... but there's no enemy there!")
            }
            CommandExecutionError::ItemNotFound => {
                write!(f, "That item was not found in the shop!")
            }
            CommandExecutionError::InsufficientFunds => write!(f, "You do not have enough gold!"),
        }
    }
}

struct CommandParser;

impl CommandParser {
    fn scan_command(&self, game_state: &GameState) -> Result<Command, CommandParsingError> {
        let base_options = vec!["Battle", "Flee", "Buy", "Sell", "Equip"];
        let command = match Select::new("What do you want to do?", base_options).prompt() {
            Ok(input) => input,
            Err(_error) => return Err(CommandParsingError::ShouldExit),
        };

        let command = match command {
            "Battle" => Command::Battle,
            "Flee" => Command::Flee,
            "Buy" => {
                let mut listing_names = game_state.get_shop_items();
                listing_names.push("Back");
                let item_name =
                    match Select::new("What do you want to buy?", listing_names).prompt() {
                        Ok(input) => input,
                        Err(_) => return Err(CommandParsingError::ShouldExit),
                    };
                if item_name == "Back" {
                    return Err(CommandParsingError::ShouldRetry);
                }
                Command::Buy(String::from(item_name))
            }
            "Sell" => {
                let mut item_names = game_state.get_inventory();
                item_names.push("Back");
                let item_name = match Select::new("What do you want to sell?", item_names).prompt()
                {
                    Ok(input) => input,
                    Err(_) => return Err(CommandParsingError::ShouldExit),
                };
                if item_name == "Back" {
                    return Err(CommandParsingError::ShouldRetry);
                }
                Command::Sell(String::from(item_name))
            }
            "Equip" => {
                let mut item_names = game_state.get_inventory();
                item_names.push("Back");
                let item_name = match Select::new("What do you want to equip?", item_names).prompt()
                {
                    Ok(input) => input,
                    Err(_) => return Err(CommandParsingError::ShouldExit),
                };
                if item_name == "Back" {
                    return Err(CommandParsingError::ShouldRetry);
                }
                Command::Equip(String::from(item_name))
            }
            _ => unreachable!(),
        };
        Ok(command)
    }
}

pub struct Game {
    game_state: GameState,
}

impl Game {
    /// Produces Game with a new Game being displayed
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
        }
    }

    /// Runs game
    pub fn run(&mut self) {
        loop {
            println!("{}", self.game_state);
            let parser = CommandParser;
            let command = match parser.scan_command(&self.game_state) {
                Ok(command) => command,
                Err(error) => match error {
                    CommandParsingError::ShouldRetry => continue,
                    CommandParsingError::ShouldExit => panic!("Input processing failed!"),
                },
            };

            let output = self.game_state.execute_command(command);
            match output {
                Err(error) => println!("{}", error),
                Ok(action) => println!("{}", action),
            }

            if self.game_state.game_over() {
                println!("Game over!");
                break;
            }
        }
    }
}
