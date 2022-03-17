use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::model::goal::*;

use super::validated::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub enabled: bool,
  pub value: Goal,
  pub on_change: Callback<Goal>
}

fn value_to_goal(value: &str) -> Goal {
  let from_i = |i: isize| {
    if (100..=999).contains(&i) { Goal::ValidManual(i) }
    else { Goal::Invalid }
  };
  let from_s = |s: &str| {
    if s.is_empty() { Goal::new() }
    else { Goal::Invalid }
  };
  match value.parse::<isize>() {
    Ok(i) => from_i(i),
    Err(_) => from_s(&value)
  }
}

#[function_component(GoalView)]
pub fn goal_view(props: &Props) -> Html {
  let value = use_state(|| "".to_string());

  let on_change = props.on_change.clone();
  let oninput = {
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
      let input: HtmlInputElement = e.target_unchecked_into();
      value.set(input.value());
      let goal = value_to_goal(&input.value());
      on_change.emit(goal);
    })
  };

  html! {
    <div class="goal">
      <Validated valid={props.value.is_valid()}>
        <input
        type="text"
        size="3"
        value={(*value).clone()}
        {oninput}
        placeholder="-auto-" />
      </Validated>
    </div>
  }
}