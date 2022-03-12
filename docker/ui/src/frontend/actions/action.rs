use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Action {
  pub caption: String,
  pub icon_class: String,
  pub enabled: bool,
  pub visible: bool,
  pub on_execute: Callback<()>,
}

impl Action {
  fn new(
    caption: &str,
    icon_class: &str,
    enabled: bool,
    visible: bool,
    on_execute: Callback<()>,
  ) -> Self {
    Action {
      caption: caption.to_string(),
      icon_class: icon_class.to_string(),
      enabled,
      visible,
      on_execute,
    }
  }

  pub fn reset_action(on_execute: Callback<()>) -> Self {
    Action::new("Reset", "fas fa-undo", false, false, on_execute)
  }

  pub fn solve_action(on_execute: Callback<()>) -> Self {
    Action::new("Solve", "fas fa-play", false, true, on_execute)
  }
}
