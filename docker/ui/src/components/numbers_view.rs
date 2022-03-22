use yew::prelude::*;

use crate::model::number::*;
use crate::model::numbers::*;
use crate::model::solution::*;

use super::number_view::*;
use super::validated::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Numbers,
  pub solution: Solution,
  pub on_change: Callback<Numbers>,
}

#[function_component(NumbersView)]
pub fn numbers_view(props: &Props) -> Html {
  let value = props.value.clone();
  let valid = value.is_valid();
  let solution = props.solution.clone();
  let use_border = value.is_individually_valid();

  html! {
      <div class="numbers">
        <datalist id="number-data">
          <option value="1" />
          <option value="2" />
          <option value="3" />
          <option value="4" />
          <option value="5" />
          <option value="6" />
          <option value="7" />
          <option value="8" />
          <option value="9" />
          <option value="10" />
          <option value="25" />
          <option value="50" />
          <option value="75" />
          <option value="100" />
        </datalist>
        <Validated {valid} {use_border} use_icon=false>
          {
            for (0..6).into_iter().map(|i| {
              let on_numbers_change = props.on_change.clone();
              let value = value.clone();
              let number = value.number(i);

              let on_number_change = Callback::from(move |n: Number| {
                let updated_numbers = value.updated(i, n);
                on_numbers_change.emit(updated_numbers.to_owned());
              });

              html!{ <NumberView value={number} solution={solution.clone()} on_change={on_number_change} /> }
            })
          }
        </Validated>
      </div>
    }
}