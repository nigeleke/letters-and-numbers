use yew::prelude::*;

#[derive(Debug)]
pub struct WarningBorder {
  props: Props,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub warn: bool,

  #[prop_or_default]
  pub children: Children,
}

impl Component for WarningBorder {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    WarningBorder { props }
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
    let class = if self.props.warn {
      "warning-border"
    } else {
      ""
    };
    html! {
      <span class=class>{ for self.props.children.iter() }</span>
    }
  }
}
