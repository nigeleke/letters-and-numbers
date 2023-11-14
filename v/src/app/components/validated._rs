use super::warning_border::*;
use super::warning_icon::*;

use yew::prelude::*;

#[derive(Debug)]
pub struct Validated {
  props: Props,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub valid: bool,

  #[prop_or(true)]
  pub use_icon: bool,

  #[prop_or(false)]
  pub use_border: bool,

  #[prop_or_default]
  pub children: Children,
}

impl Component for Validated {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Validated { props }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    unimplemented!()
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    // Force children to render...
    true
  }

  fn view(&self) -> Html {
    let border_warning = !self.props.valid && self.props.use_border;
    let icon_warning = !self.props.valid && self.props.use_icon;
    html! {
      <span class="validated">
        <WarningBorder warn=border_warning>
          { for self.props.children.iter() }
        </WarningBorder>
        <WarningIcon warn=icon_warning />
      </span>
    }
  }
}
