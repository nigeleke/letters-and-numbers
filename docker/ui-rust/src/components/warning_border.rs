use style4rs::style;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub warn: bool,

  #[prop_or_default]
  pub children: Children,
}

#[function_component(WarningBorder)]
pub fn warning_border(props: &Props) -> Html {
  let class = style! {
    span {
      border-color: gold;
      border-width: 5;
      border-style: solid;
    }
  };

  let class = if props.warn { class } else { "" };

  html! {
    <span class={class}>{ for props.children.iter() }</span>
  }
}