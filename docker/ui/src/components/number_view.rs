use web_sys::HtmlInputElement;
// use super::validated::*;
//
// use crate::shared::number::*;
//
use yew::prelude::*;

use crate::model::number::*;

use super::validated::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub enabled: bool,
  pub value: Number,
  pub on_change: Callback<Number>,
}

fn value_to_number(value: &str) -> Number {
  let from_i = |i: isize| {
    if (1..=10).contains(&i) || vec![25, 50, 75, 100].contains(&i) { Number::Valid(i) }
    else { Number::Invalid }
  };
  match value.parse::<isize>() {
    Ok(i) => from_i(i),
    Err(_) => Number::Invalid
  }
}

#[function_component(NumberView)]
pub fn number_view(props: &Props) -> Html {
  let value = use_state(|| "".to_string());

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

  html! {
    <div class="number">
      <Validated valid={props.value.is_valid()} use_icon=true >
        <input
          type="text"
          size="2"
          list="number-data"
          value={(*value).clone()}
          {oninput}
          autocomplete="off" />
      </Validated>
    </div>
  }

}