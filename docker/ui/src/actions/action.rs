use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Action {
  pub caption: String,
  pub icon_class: String,
  pub enabled: bool,
  pub visible: bool,
  pub on_execute: Callback<()>,
}

impl Action {
  pub fn undefined() -> Self {
    Action::new("", "", false, false, Callback::noop())
  }

  pub fn new(
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
}
