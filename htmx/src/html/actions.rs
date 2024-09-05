use crate::domain::{Goal, Numbers};
use crate::state::AppState;

use axum::extract::State;
use axum_htmx::HxResponseTrigger;
use maud::{html, Markup};

// pub fn solve_action_markup(goal: &Goal, numbers: &Numbers) -> Markup {
//     println!("solve_action goal {:?} numbers {:?}", goal, numbers);
//     let disabled = !(goal.is_valid() && numbers.is_valid());
//     html! {
        
//             button
//                 id="solve-button"
//                 disabled=(disabled)
//                 hx-post="/reveal_goal"
//                 hx-swap="none"
//                 { "Solve" }
//     }
// }

pub async fn actions(state: State<AppState>) -> Markup {
    let state = state.lock().unwrap();
    let goal = state.goal();
    let numbers = state.numbers();
    println!("solve_action::state: {:?}", state);
    html!{}
}

pub async fn reveal_goal(State(state): State<AppState>) -> (HxResponseTrigger, ()) {
    let mut state = state.lock().unwrap();

    state.reveal_goal();

    (HxResponseTrigger::normal(["goal-revealed"]), ())
}

pub async fn get_solution() -> (HxResponseTrigger, ()) {
    (HxResponseTrigger::normal(["solution-revealed"]), ())
}
