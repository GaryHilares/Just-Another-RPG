use crate::game::Game;

mod action_summary;
mod encounter;
mod enemy;
mod game;
mod game_state;
mod item;
mod player;

fn main() {
    let mut game = Game::new();
    game.run();
}
