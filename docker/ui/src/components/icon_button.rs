use yew::prelude::*;

use crate::actions::action::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub action: Action,
}

#[function_component(IconButton)]
pub fn icon_button(props: &Props) -> Html {
  let action = props.action.clone();
  let on_execute = action.on_execute;
  let onclick = move |_: MouseEvent| { on_execute.emit(()); };
  html! {
    <button
      class="icon-button"
      type="button"
      disabled={!action.enabled}
      hidden={!action.visible}
      {onclick}>
      <span class={action.icon_class.to_string()} />
    </button>
  }
}
