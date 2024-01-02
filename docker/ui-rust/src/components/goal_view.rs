use style4rs::style_sheet;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::model::goal::*;
use crate::model::solution::*;

use super::validated::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Goal,
  pub solution: Solution,
  pub on_change: Callback<Goal>,
}

fn value_to_goal(value: &str) -> Goal {
  let from_i = |i: isize| {
    if (100..=999).contains(&i) {
      Goal::from(i)
    } else {
      Goal::undefined()
    }
  };
  let from_s = |s: &str| {
    if s.is_empty() {
      Goal::auto()
    } else {
      Goal::undefined()
    }
  };
  match value.parse::<isize>() {
    Ok(i) => from_i(i),
    Err(_) => from_s(value),
  }
}

#[function_component(GoalView)]
pub fn goal_view(props: &Props) -> Html {
  let class = style_sheet!("css/goal_view.css");

  let value = use_state_eq(|| "".to_owned());
  match props.solution {
    Solution::Unsolved => (),
    Solution::Reset => value.set("".to_owned()),
    _ => value.set(props.value.to_string()),
  }

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

  let enabled = props.solution == Solution::Unsolved;

  html! {
    <div class={class}>
      <Validated valid={props.value.is_valid()}>
        <input
        type="text"
        size="3"
        value={(*value).clone()}
        {oninput}
        disabled={!enabled}
        placeholder="-auto-" />
      </Validated>
    </div>
  }
}
