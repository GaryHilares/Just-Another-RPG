use crate::controller::GameController;

mod controller;
mod model;

fn main() {
    let mut game = GameController::new();
    game.run();
}
