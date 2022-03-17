use yew::prelude::*;

use crate::validated::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub enabled: bool,
  pub valid: bool,
}

#[function_component(GoalView)]
pub fn goal_view(props: &Props) -> Html {
  html! {
    <div class="goal">
      <Validated valid={props.valid}>
        <input
        type="text"
        size="3"
        placeholder="-auto-" />
      </Validated>
    </div>
  }
}