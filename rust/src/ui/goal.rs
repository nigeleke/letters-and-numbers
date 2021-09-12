use crate::ui::validate::*;

use rand::distributions::{Distribution, Uniform};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GoalProps {
  pub value: GoalValue,
  pub on_update: Callback<GoalValue>,
  pub active: bool,
  pub revealed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GoalValue {
  Auto(i32),
  Manual(Option<i32>),
}

#[derive(Debug)]
pub enum GoalMsg {
  Validate(String),
}

pub struct Goal {
  props: GoalProps,
  link: ComponentLink<Self>,
  displayed_value: String,
}

impl Goal {
  fn validate(s: &str) -> GoalValue {
    fn valid_target(i: i32) -> GoalValue {
      let is_valid = 100 <= i && i <= 999;
      if is_valid {
        GoalValue::Manual(Some(i))
      } else {
        GoalValue::Manual(None)
      }
    }
    fn valid_auto(s: &str) -> GoalValue {
      if s.len() == 0 {
        Goal::random_auto()
      } else {
        GoalValue::Manual(None)
      }
    }
    match s.parse::<i32>() {
      Ok(i) => valid_target(i),
      Err(_) => valid_auto(s),
    }
  }

  fn is_valid(&self) -> bool {
    return self.props.value != GoalValue::Manual(None);
  }

  pub fn random_auto() -> GoalValue {
    let mut rng = rand::thread_rng();
    let target = Uniform::from(100..1000);
    GoalValue::Auto(target.sample(&mut rng))
  }
}

impl Component for Goal {
  type Message = GoalMsg;
  type Properties = GoalProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Goal {
      props,
      link,
      displayed_value: "".to_string(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      GoalMsg::Validate(s) => {
        let goal = Goal::validate(&s);
        self.props.on_update.emit(goal);
        self.displayed_value = match (goal, self.props.revealed) {
          (GoalValue::Auto(target), true) => target.to_string(),
          (GoalValue::Auto(_), false) => "".to_string(),
          (GoalValue::Manual(_), _) => s.to_string(),
        };
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    let changed = self.props != props;
    if props.revealed {
      self.displayed_value = match props.value {
        GoalValue::Auto(target) => target.to_string(),
        GoalValue::Manual(Some(target)) => target.to_string(),
        GoalValue::Manual(None) => self.displayed_value.to_string(),
      }
    }
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
              value=self.displayed_value.to_string() />
          </ValidateItem>
        </Validate>
      </div>
    }
  }
}
