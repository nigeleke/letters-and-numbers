use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
  pub warn: bool,
}

#[derive(Debug)]
pub struct WarningIcon {
  props: Props,
}

impl Component for WarningIcon {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self { props }
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
    let class = if self.props.warn { "warning-icon" } else { "" };
    html! {
      <span class=class />
    }
  }
}
