use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    numbers: Vec<i32>,
    goal: i32,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            numbers: vec![],
            goal: 0,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("w3-theme")>
                <header class=classes!("w3-center", "w3-theme-dl", "w3-xxlarge")>{ "Letters and Numbers" }</header>
                <div class=classes!("w3-center")>{ "Desktop" }</div>
                <footer class=classes!("w3-bottom", "w3-center", "w3-theme-d3", "w3-tiny")>{ "Copyright (C) 2021; Nigel Eke. All rights reserved." }</footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
