use super::game_state::GameState;

use crate::domain::Numbers;

use axum::extract::FromRef;

pub struct NumbersState(Numbers);

impl FromRef<GameState> for NumbersState {
    fn from_ref(input: &GameState) -> Self {
        NumbersState(input.numbers())
    }
}
