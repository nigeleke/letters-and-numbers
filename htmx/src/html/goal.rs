use crate::domain::Goal;
use crate::state::AppState;

use axum::extract::State;
use axum_htmx::{HxResponseTrigger, HxTriggerName};
use maud::{html, Markup};

pub fn goal(goal: &Goal) -> Markup {
    html! {
        input type="text"
            disabled
            value=(goal.to_string()) {}
    }
}

pub fn goal_edit(goal: &mut Goal) -> Markup {
    html! {
        div .validated {
            input type="text"
                value=(goal.to_string())
                name="value"
                placeholder="-auto-"
                size="3"
                hx-post="/update_goal"
                hx-trigger="change, keyup delay:500ms changed"
                hx-target="next span"
                hx-swap="outerHTML" {}
            (goal_validation(goal))
        }
    }
}

fn goal_validation(goal: &mut Goal) -> Markup {
    if goal.is_valid() {
        html! { span .validated {} }
    } else {
        html! { span .validated .warning {} }
    }
}

pub async fn update_goal(
    State(state): State<AppState>,
    HxTriggerName(name): HxTriggerName,
    body: String,
) -> (HxResponseTrigger, Markup) {
    let name = name.unwrap_or_default();
    let value = body
        .trim_start_matches(&format!("{}=", name))
        .trim()
        .to_string();

    let mut goal = Goal::from(value);

    let mut state = state.lock().unwrap();
    state.update_goal(&goal);
    (
        HxResponseTrigger::normal(["goal-updated"]),
        goal_validation(&mut goal),
    )
}
