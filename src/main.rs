//! Entrypoint to the program. Runs a new game of Just another RPG.
use crate::controller::GameController;

mod controller;
mod model;

/// Entrypoint to the program. Runs the game.
fn main() {
    let mut game = GameController::new();
    game.run();
}
