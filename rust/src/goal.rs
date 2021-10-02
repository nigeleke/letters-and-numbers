use crate::validate::*;

use rand::distributions::{Distribution, Uniform};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GoalProps {
  pub value: GoalValue,
  pub active: bool,
  pub on_update: Callback<GoalValue>,
  pub revealed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GoalValue {
  Auto(i32),
  Manual(i32),
  Invalid,
}

impl GoalValue {
  pub fn is_valid(&self) -> bool {
    !matches!(self, GoalValue::Invalid)
  }
}

#[derive(Debug)]
pub enum GoalMsg {
  Validate(String),
}

pub struct Goal {
  props: GoalProps,
  link: ComponentLink<Self>,
  value_impl: String,
}

impl Goal {
  fn value_to_impl(value: GoalValue) -> String {
    match value {
      GoalValue::Auto(i) => i.to_string(),
      GoalValue::Manual(i) => i.to_string(),
      GoalValue::Invalid => "".into(),
    }
  }

  fn validate(s: &str) -> GoalValue {
    fn validated_manual(i: i32) -> GoalValue {
      let is_valid = (100..=999).contains(&i);
      if is_valid {
        GoalValue::Manual(i)
      } else {
        GoalValue::Invalid
      }
    }
    fn validated_auto(s: &str) -> GoalValue {
      if s.is_empty() {
        Goal::random_auto()
      } else {
        GoalValue::Invalid
      }
    }
    match s.parse::<i32>() {
      Ok(i) => validated_manual(i),
      Err(_) => validated_auto(s),
    }
  }

  fn is_valid(&self) -> bool {
    !matches!(self.props.value, GoalValue::Invalid)
  }

  pub fn random_auto() -> GoalValue {
    let mut rng = rand::thread_rng();
    let target = Uniform::from(100..=999);
    GoalValue::Auto(target.sample(&mut rng))
  }

  fn as_revealed(&self) -> String {
    match &self.props.value {
      GoalValue::Auto(i) => i.to_string(),
      GoalValue::Manual(i) => i.to_string(),
      GoalValue::Invalid => self.value_impl.to_string(),
    }
  }
}

impl Component for Goal {
  type Message = GoalMsg;
  type Properties = GoalProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let value_impl = Goal::value_to_impl(props.value);
    Goal {
      props,
      link,
      value_impl,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      GoalMsg::Validate(s) => {
        let goal = Goal::validate(&s);
        self.props.on_update.emit(goal);
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
      <div class=classes!("w3-center", "w3-section")>
        <Validate>
          <ValidateItem is_valid=self.is_valid()>
            <input
              class=classes!("goal", "w3-round-large", "w3-theme-l3", "w3-xxxlarge")
              disabled=!self.props.active
              hidden=self.props.revealed
              size="3"
              placeholder="auto"
              title="Target number"
              oninput=self.link.callback(|event: InputData| GoalMsg::Validate(event.value)) />
            <input
              class=classes!("goal", "w3-round-large", "w3-theme-l3", "w3-xxxlarge")
              disabled=true
              readonly=true
              hidden=!self.props.revealed
              size="3"
              title="Target number"
              value=self.as_revealed() />
          </ValidateItem>
        </Validate>
      </div>
    }
  }
}
