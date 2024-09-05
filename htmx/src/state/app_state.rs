use super::game_state::GameState;

use std::sync::{Arc, Mutex};

pub type AppState = Arc<Mutex<GameState>>;

pub fn initial_state() -> AppState {
    Arc::new(Mutex::new(GameState::default()))
}
