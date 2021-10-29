mod frontend {
  pub mod actions {
    pub mod action;
  }
  pub mod components {
    mod goal_view;
    mod icon_button;
    mod main_app;
    mod number_view;
    mod numbers_view;
    mod solution_view;
    mod validated;
    mod warning_border;
    mod warning_icon;
    pub use main_app::MainApp;
  }
}

mod shared {
  pub mod goal;
  pub mod number;
  pub mod numbers;
  pub mod solution;
}

mod api {
  pub mod resolver_service;
}

pub mod backend {
  pub mod core {
    mod expression;
    mod operator;
    pub mod resolver;
  }
}

use frontend::components::*;

use yew::prelude::*;

#[derive(Debug)]
pub struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    unimplemented!()
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <>
        <header>{"Letters and Numbers"}</header>
        <main>
          <MainApp />
        </main>
        <footer>{"Copyright © Nigel Eke; All rights reserved."}</footer>
      </>
    }
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
