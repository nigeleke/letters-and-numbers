use super::number::{number, number_edit};

use crate::{domain::Numbers, state::AppState};

use axum::extract::State;
use axum_htmx::HxResponseTrigger;
use maud::{html, Markup};

pub fn numbers(numbers: &Numbers) -> Markup {
    html! {
        div .numbers {
            @for n in numbers.as_ref() {
                (number(n))
            }
        }
    }
}

pub fn numbers_edit(numbers: &Numbers) -> Markup {
    let enumerated_numbers = numbers.as_ref().iter().enumerate();
    html! {
        div .validated .numbers-outer {
            div .numbers-inner 
                hx-trigger="number-updated"
                hx-post="/validate_numbers"
                hx-target="next span"
                hx-swap="outerHTML" {
                @for (i, n) in enumerated_numbers {
                    (number_edit(n, i))
                }
            }
            (numbers_validation(numbers))
        }

    }
}

fn numbers_validation(numbers: &Numbers) -> Markup {
    if numbers.is_any_number_invalid() || !numbers.is_combination_invalid() {
        html! { span .validated {} }
    } else {
        html! { span .validated .warning {} }
    }
}

pub async fn validate_numbers(
    State(state): State<AppState>
) -> (HxResponseTrigger, Markup) {
    let state = state.lock().unwrap();
    println!("validate_numbers::state: {:?}", state);
    (
        HxResponseTrigger::normal(["numbers-updated"]),
        numbers_validation(&state.numbers())
    )
}
