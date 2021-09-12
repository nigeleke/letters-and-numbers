use crate::ui::validate::*;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NumberProps {
  pub active: bool,
  pub on_update: Callback<Option<i32>>,
}

#[derive(Debug)]
pub enum NumberMsg {
  Validate(String),
}

#[derive(Debug)]
pub struct Number {
  props: NumberProps,
  link: ComponentLink<Self>,
  maybe_number: Option<i32>,
}

impl Number {
  fn validate(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
      Ok(i) => {
        let is_valid = 1 <= i && i <= 10 || i == 25 || i == 50 || i == 75 || i == 100;
        if is_valid {
          Some(i)
        } else {
          None
        }
      }
      Err(_) => None,
    }
  }

  fn is_valid(&self) -> bool {
    return self.maybe_number.is_some();
  }
}

impl Component for Number {
  type Message = NumberMsg;
  type Properties = NumberProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Number {
      props,
      link,
      maybe_number: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      NumberMsg::Validate(s) => {
        self.maybe_number = Number::validate(&s);
        self.props.on_update.emit(self.maybe_number);
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
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
          oninput=self.link.callback(|event: InputData| NumberMsg::Validate(event.value)) />
        </ValidateItem>
      </Validate>
    }
  }
}
