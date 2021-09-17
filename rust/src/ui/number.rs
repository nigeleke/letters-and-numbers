use crate::ui::validate::*;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NumberProps {
  pub value: NumberValue,
  pub active: bool,
  pub on_update: Callback<NumberValue>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberValue {
  Unset,
  Valid(i32),
  Invalid,
}

impl NumberValue {
  pub fn trivial_value(&self) -> i32 {
    match self {
      NumberValue::Valid(i) => *i,
      _ => 0
    }
  }
}

#[derive(Debug)]
pub enum NumberMsg {
  Validate(String),
}

#[derive(Debug)]
pub struct Number {
  props: NumberProps,
  link: ComponentLink<Self>,
  value_impl: String,
}

impl Number {
  fn value_to_impl(value: NumberValue) -> String {
    match value {
      NumberValue::Valid(i) => i.to_string(),
      _ => "".into(),
    }
  }

  fn validate(s: &str) -> NumberValue {
    match s.parse::<i32>() {
      Ok(i) => {
        let is_valid = (1..=10).contains(&i) || i == 25 || i == 50 || i == 75 || i == 100;
        if is_valid {
          NumberValue::Valid(i)
        } else {
          NumberValue::Invalid
        }
      }
      Err(_) => NumberValue::Invalid,
    }
  }

  fn is_valid(&self) -> bool {
    matches!(self.props.value, NumberValue::Valid(_))
  }
}

impl Component for Number {
  type Message = NumberMsg;
  type Properties = NumberProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let value_impl = Number::value_to_impl(props.value);
    Number {
      props,
      link,
      value_impl,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      NumberMsg::Validate(s) => {
        self.props.value = Number::validate(&s);
        self.value_impl = s;
        self.props.on_update.emit(self.props.value);
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if props.value == NumberValue::Unset {
      self.value_impl = "".into();
    }
    let changed = self.props != props;
    self.props = props;
    changed
  }

  fn view(&self) -> Html {
    html! {
      <Validate>
        <ValidateItem is_valid=self.is_valid()>
          <input
          class=classes!("number", "w3-round-large", "w3-theme-l3", "w3-xxlarge")
          disabled=!self.props.active
          size="2"
          title="Number"
          oninput=self.link.callback(|event: InputData| NumberMsg::Validate(event.value))
          value=self.value_impl.to_string() />
        </ValidateItem>
      </Validate>
    }
  }
}
