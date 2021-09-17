use crate::ui::number::*;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NumbersProps {
  pub value: NumbersValue,
  pub active: bool,
  pub on_update: Callback<NumbersValue>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumbersValue {
  Unset,
  Valid(i32, i32, i32, i32, i32, i32),
  Invalid,
}

impl NumbersValue {
  pub fn is_valid(&self) -> bool {
    matches!(self, NumbersValue::Valid(_, _, _, _, _, _))
  }
}

#[derive(Debug)]
pub enum NumbersMsg {
  ValidateNumber(usize, NumberValue),
}

pub struct Numbers {
  props: NumbersProps,
  link: ComponentLink<Self>,
  value_impl: Vec<NumberValue>,
}

impl Numbers {
  fn value_to_impl(value: NumbersValue) -> Vec<NumberValue> {
    match value {
      NumbersValue::Unset => vec![NumberValue::Unset; 6],
      NumbersValue::Valid(i0, i1, i2, i3, i4, i5) => vec![
        NumberValue::Valid(i0),
        NumberValue::Valid(i1),
        NumberValue::Valid(i2),
        NumberValue::Valid(i3),
        NumberValue::Valid(i4),
        NumberValue::Valid(i5),
      ],
      NumbersValue::Invalid => vec![NumberValue::Invalid; 6],
    }
  }
}

impl Component for Numbers {
  type Message = NumbersMsg;
  type Properties = NumbersProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let value_impl = Numbers::value_to_impl(props.value);
    Numbers {
      props,
      link,
      value_impl,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    fn valid_set(ns: &[i32]) -> bool {
      let lows1 = 1..=10;
      let lows2 = 1..=10;
      let highs = (25..=100).step_by(25);
      let allowed_numbers_iter = lows1.chain(lows2).chain(highs);
      let mut allowed_numbers = allowed_numbers_iter.collect::<Vec<i32>>();
      allowed_numbers.sort_unstable();
      let remaining_numbers = ns.iter().fold(allowed_numbers, |mut agg, n| {
        let maybe_i = agg.binary_search(n);
        match maybe_i {
          Ok(i) => {
            agg.remove(i);
            agg
          }
          Err(_) => agg,
        }
      });
      remaining_numbers.len() == 18
    }
    match msg {
      NumbersMsg::ValidateNumber(id, number_value) => {
        self.value_impl[id] = number_value;
        let is = self
          .value_impl
          .iter()
          .map(|nv| nv.trivial_value())
          .collect::<Vec<i32>>();
        let validated_numbers = if is.len() == 6 && valid_set(&is) {
          NumbersValue::Valid(is[0], is[1], is[2], is[3], is[4], is[5])
        } else {
          NumbersValue::Invalid
        };
        self.props.on_update.emit(validated_numbers);
        true
      },
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if props.value == NumbersValue::Unset {
      self.value_impl = Numbers::value_to_impl(props.value);
    }
    let changed = self.props != props;
    self.props = props;
    changed
  }

  fn view(&self) -> Html {
    let cb = |id| move |update| NumbersMsg::ValidateNumber(id, update);
    let active = self.props.active;
    html! {
      <div class=classes!("w3-center", "w3-section", "w3-auto", "w3-row", "w3-row-padding")>
        { for (0..6).into_iter().map(|i| html!{<Number value=self.value_impl[i] on_update=self.link.callback(cb(i)) active=active />}) }
      </div>
    }
  }
}
