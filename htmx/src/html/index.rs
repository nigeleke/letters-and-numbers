use super::base::base;
use super::goal::{goal, goal_edit};
use super::numbers::{numbers, numbers_edit};
use super::solve_action::solve_action;

use crate::domain::{Goal, Numbers, Solution};
use crate::state::{AppState, GameState};

use axum::extract::State;
use maud::{html, Markup};

pub async fn index(State(state): State<AppState>) -> Markup {
    let mut state = state.lock().unwrap();
    base(html! {
        header { "Letters & Numbers" }
        main { (app_state_markup(&mut *state)) }
        footer { "Copyright (c) 2024, Nigel Eke. All rights reserved."}
    })
}

fn app_state_markup(state: &mut GameState) -> Markup {
    match state {
        GameState::Entering(goal, numbers) => entering(goal, numbers),
        GameState::Solving(goal, numbers) => solving(goal, numbers),
        _ => html! {},
        // AppState::Solved(goal, numbers, solution) => solved(goal, numbers, solution),
    }
}

fn entering(goal: &mut Goal, numbers: &mut Numbers) -> Markup {
    template(
        goal_edit(goal),
        numbers_edit(numbers),
        html! {},
        html! {},
    )
}

fn solving(goal: &Goal, numbers: &Numbers) -> Markup {
    template(
        goal(goal),
        numbers(numbers),
        html! {},
        html! {},
    )
}

// fn solved(goal: Goal, numbers: Numbers, solution: Solution) -> Markup {
//     html! {}
// }

fn template(goal: Markup, numbers: Markup, actions: Markup, solution: Markup) -> Markup {
    html! {
        div hx-trigger="numbers-updated; goal-updated"
            hx-post="/actions"
            hx-target="#actions" {
                (goal)
                (numbers)
                (actions)
            }
        (solution)
    }
}
