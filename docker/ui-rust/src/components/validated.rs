use style4rs::style;
use yew::prelude::*;

use super::warning_border::*;
use super::warning_icon::*;

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

#[function_component(Validated)]
pub fn validated(props: &Props) -> Html {
  let class = style! {
    span {
      display: flex;
      flex-direction: row;
    }
  };
  
  let border_warning = !props.valid && props.use_border;
  let icon_warning = !props.valid && props.use_icon;

  html! {
      <span class={class}>
        <WarningBorder warn={border_warning}>
          { for props.children.iter() }
        </WarningBorder>
        <WarningIcon warn={icon_warning} />
      </span>
    }
}