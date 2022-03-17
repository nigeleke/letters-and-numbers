mod components;
mod model;

use yew::prelude::*;

use crate::components::desktop::*;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <>
      <header>{"Letters and Numbers"}</header>
      <main>
        <Desktop />
      </main>
      <footer>{"Copyright © 2022; Nigel Eke - All rights reserved."}</footer>
    </>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
