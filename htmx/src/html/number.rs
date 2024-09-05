use crate::domain::Number;
use crate::state::AppState;

use axum::extract::{Path, State};
use axum_htmx::{HxResponseTrigger, HxTriggerName};
use maud::{html, Markup};

pub fn number(number: &Number) -> Markup {
    html! {
        div .validated {
            input type="text"
                value=(number.to_string())
                name="value"
                size="2"
                disabled {}
        }
    }
}

pub fn number_edit(number: &Number, index: usize) -> Markup {
    html! {
        div .validated {
            input type="text"
                value=(number.to_string())
                name="value"
                size="2"
                hx-post=(format!("/update_number/{}", index))
                hx-trigger="change, keyup delay:500ms changed"
                hx-target="next span"
                hx-swap="outerHTML" {}
            (number_validation(number))
        }
    }
}

fn number_validation(number: &Number) -> Markup {
    if number.is_valid() {
        html! { span .validated {} }
    } else {
        html! { span .validated .warning {} }
    }
}

pub async fn update_number(
    Path(index): Path<usize>,
    State(state): State<AppState>,
    HxTriggerName(name): HxTriggerName,
    body: String,
) -> (HxResponseTrigger, Markup) {
    let name = name.unwrap_or_default();
    let value = body
        .trim_start_matches(&format!("{}=", name))
        .trim()
        .to_string();

    let number = Number::from(value);

    let mut state = state.lock().unwrap();
    state.update_number(index, &number);
    (
        HxResponseTrigger::normal(["number-updated"]),
        number_validation(&number),
    )
}
