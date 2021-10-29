use super::validated::*;

use crate::shared::goal::*;

use yew::prelude::*;

#[derive(Debug)]
pub struct GoalView {
  props: Props,
  link: ComponentLink<Self>,
  string_value: String,
}

#[derive(Debug)]
pub enum Msg {
  Updated(String),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Goal,
  pub on_change: Callback<Goal>,
}

impl Component for GoalView {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link,
      string_value: "".to_owned(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    let from_i = |i: isize| {
      if (100..=999).contains(&i) {
        Goal::ValidManual(i)
      } else {
        Goal::Invalid
      }
    };
    let from_s = |s: &str| {
      if s.is_empty() {
        Goal::ValidAuto(999)
      } else {
        Goal::Invalid
      }
    };
    match msg {
      Msg::Updated(v) => {
        let goal = match v.parse::<isize>() {
          Ok(i) => from_i(i),
          Err(_) => from_s(&v),
        };
        let notify = goal != self.props.value;
        if notify {
          self.props.on_change.emit(goal);
          self.string_value = v;
        }
        false
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    let render = self.props != props;

    match props.value {
      Goal::ValidAuto(_) => self.string_value = "".to_owned(),
      Goal::ValidManual(v) => self.string_value = v.to_string(),
      _ => (),
    };

    self.props = props;

    render
  }

  fn view(&self) -> Html {
    let oninput = self
      .link
      .callback(|event: InputData| Msg::Updated(event.value));
    html! {
      <div class="goal">
        <Validated valid=self.props.value.is_valid()>
          <input
          type="text"
          size="3"
          oninput=oninput
          value=self.string_value.to_string()
          placeholder="-auto-" />
        </Validated>
      </div>
    }
  }
}
