use style4rs::style;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
  pub warn: bool,
}

#[function_component(WarningIcon)]
pub fn warning_icon(props: &Props) -> Html {
  let class = style! {
    span::after {
      content: "⚠️";
      font-size: 0.75rem;
      vertical-align: text-top;
      margin-left: -1.25rem;
    }   
  };

  let class = if props.warn { class } else { "" };

  html! {
    <span class={class} />
  }
}