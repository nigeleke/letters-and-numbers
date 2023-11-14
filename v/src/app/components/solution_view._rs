use crate::shared::solution::*;

use yew::prelude::*;

#[derive(Debug)]
pub struct SolutionView {
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Solution,
}

impl Component for SolutionView {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { props, link }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    let render = self.props != props;
    self.props = props;
    render
  }

  fn view(&self) -> Html {
    html! {
      <div class="solution">{self.props.value.to_owned()}</div>
    }
  }
}
