use yew::prelude::*;

use crate::model::number::*;
use crate::model::numbers::*;

use super::number_view::*;
use super::validated::*;

use itertools::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub enabled: bool,
  pub value: Numbers,
  pub on_change: Callback<Numbers>,
}

fn validate_each(ns: &[Number]) -> bool {
  let valid_numbers: Vec<&Number> = ns.iter().filter(|n| n.is_valid()).collect();
  valid_numbers.len() == 6
}

fn validate_group(ns: &[Number]) -> bool {
  fn value_of(n: &Number) -> isize {
    match *n {
      Number::Valid(i) => i,
      Number::Invalid => 0
    }
  }

  let number_values: Vec<isize> = ns.iter().map(|n| value_of(n)).collect();
  let mut counts = number_values.iter().counts_by(|n| n);

  counts.retain(|i, count| {
    if (1..=10).contains(*i) { *count > 2 }
    else if vec![25, 50, 75, 100].contains(*i) { *count > 1 }
    else { true }
  });

  counts.is_empty()
}

fn vec_to_numbers(ns: &[Number]) -> Numbers {
  let is_valid = validate_group(&ns);
  if is_valid { Numbers::Valid(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]) }
  else { Numbers::Invalid(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]) }
}

fn numbers_to_vec(numbers: &Numbers) -> Vec<Number> {
  match *numbers {
    Numbers::Valid(n1, n2, n3, n4, n5, n6) => vec![n1, n2, n3, n4, n5, n6],
    Numbers::Invalid(n1, n2, n3, n4, n5, n6) => vec![n1, n2, n3, n4, n5, n6]
  }
}

#[function_component(NumbersView)]
pub fn numbers_view(props: &Props) -> Html {
  let value = props.value;
  let valid = value.is_valid();
  let value_vec = numbers_to_vec(&value);
  let use_border = validate_each(&value_vec);

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
              let value_vec = numbers_to_vec(&value);
              let number = value_vec[i];

              let on_number_change = Callback::from(move |n: Number| {
                log::info!("number {:?} changed {:?}", i, n);
                let mut updated_numbers_vec = value_vec.clone();
                updated_numbers_vec[i] = n;
                let updated_numbers = vec_to_numbers(&updated_numbers_vec);
                on_numbers_change.emit(updated_numbers);
              });

              html!{ <NumberView enabled={props.enabled} value={number} on_change={on_number_change} /> }
            })
          }
        </Validated>
      </div>
    }
}