mod core;
mod ui;

use ui::desktop::*;

use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <header class=classes!("w3-center", "w3-theme-dl", "w3-xxlarge")>{ "Letters and Numbers" }</header>
                <Desktop />
                <footer class=classes!("w3-bottom", "w3-center", "w3-theme-d3", "w3-tiny")>{ "Copyright (C) 2021; Nigel Eke. All rights reserved." }</footer>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
