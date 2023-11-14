use super::number_view::*;
use super::validated::*;

use crate::shared::number::*;
use crate::shared::numbers::*;

use yew::prelude::*;

#[derive(Debug)]
pub struct NumbersView {
  pub props: Props,
  pub link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum Msg {
  NumberUpdated(usize, Number),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Numbers,
  pub on_change: Callback<Numbers>,
}

impl Component for NumbersView {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { props, link }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    fn is_valid_set(ns: &[Number]) -> bool {
      let mut counts = vec![0; 101];
      for n in ns {
        if let Number::Valid(i) = n {
          let count_index = *i as usize;
          counts[count_index] += 1;
        }
      }

      let sum: usize = counts.iter().sum();

      let is_valid_count = |index: usize, max: usize| (counts[index] <= max) as usize;

      let valid_lows: usize = (1..=10).map(|i| is_valid_count(i, 2)).sum();
      let valid_highs: usize = (25..=100).step_by(25).map(|i| is_valid_count(i, 1)).sum();

      sum == 6 && valid_lows == 10 && valid_highs == 4
    }

    let as_updated_numbers = |i, n| {
      let mut ns = match self.props.value.clone() {
        Numbers::Valid(numbers) => numbers,
        Numbers::Invalid(numbers) => numbers,
      };
      ns[i] = n;
      if is_valid_set(&ns) {
        Numbers::Valid(ns)
      } else {
        Numbers::Invalid(ns)
      }
    };

    match msg {
      Msg::NumberUpdated(i, number) => {
        let numbers = as_updated_numbers(i, number);
        let notify = numbers != self.props.value;
        if notify {
          self.props.on_change.emit(numbers);
        };
        false
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    let render = self.props != props;
    self.props = props;
    render
  }

  fn view(&self) -> Html {
    let on_change_number = |i| {
      self
        .link
        .callback(move |number| Msg::NumberUpdated(i, number))
    };

    let numbers = match self.props.value.clone() {
      Numbers::Valid(numbers) => numbers,
      Numbers::Invalid(numbers) => numbers,
    };

    let valid_numbers_count: isize = numbers
      .iter()
      .map(|number| number.is_valid() as isize)
      .sum();
    let valid = if valid_numbers_count < 6 {
      true
    } else {
      self.props.value.is_valid()
    };

    html! {
      <div class="numbers">
        <Validated valid=valid use_border=true use_icon=false>
          {
            for (0..6).into_iter().map(|i| {
              let number = numbers[i];
              html!{ <NumberView value=number on_change=on_change_number(i) /> }
            })
          }
        </Validated>
      </div>
    }
  }
}
