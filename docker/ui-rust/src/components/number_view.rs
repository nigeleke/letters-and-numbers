use style4rs::style_sheet;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::model::number::*;
use crate::model::solution::Solution;

use super::validated::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Number,
  pub solution: Solution,
  pub on_change: Callback<Number>,
}

fn value_to_number(value: &str) -> Number {
  let from_i = |i: isize| {
    if (1..=10).contains(&i) || [25, 50, 75, 100].contains(&i) {
      Number::from(i)
    } else {
      Number::undefined()
    }
  };
  match value.parse::<isize>() {
    Ok(i) => from_i(i),
    Err(_) => Number::undefined(),
  }
}

#[function_component(NumberView)]
pub fn number_view(props: &Props) -> Html {
  let class = style_sheet!("css/number_view.css");

  let value = use_state_eq(|| "".to_string());
  if props.solution == Solution::Reset {
    value.set("".to_owned())
  }

  let on_change = props.on_change.clone();

  let oninput = {
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
      let input: HtmlInputElement = e.target_unchecked_into();
      value.set(input.value());
      let number = value_to_number(&input.value());
      on_change.emit(number);
    })
  };

  let enabled = props.solution == Solution::Unsolved;

  html! {
    <div class={class}>
      <Validated valid={props.value.is_valid()} use_icon=true >
        <input
          class={class}
          type="text"
          size="2"
          list="number-data"
          value={(*value).clone()}
          {oninput}
          disabled={!enabled}
          autocomplete="off" />
      </Validated>
    </div>
  }
}
