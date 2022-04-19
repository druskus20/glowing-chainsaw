use bevy::prelude::{App, Plugin};

pub struct GameStatePlugin;

impl GameStatePlugin {
    pub fn new() -> Self {
        GameStatePlugin {}
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Game);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Game,
}
