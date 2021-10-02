use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SolutionProps {
    pub enable_solve: bool,
    pub solution: String,
    pub on_action: Callback<SolutionMsg>,
}

#[derive(Debug)]
pub enum SolutionMsg {
    Solve,
    Reset,
}

pub struct Solution {
    props: SolutionProps,
    link: ComponentLink<Self>,
}

impl Solution {
    fn have_solution(&self) -> bool {
        !self.props.solution.is_empty()
    }
}

impl Component for Solution {
    type Message = SolutionMsg;
    type Properties = SolutionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Solution { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.on_action.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props != props;
        self.props = props;
        changed
    }

    fn view(&self) -> Html {
        html! {
          <div class=classes!("w3-center", "w3-section", "w3-xlarge")>
            <div>
              <button
                title="Find solution"
                hidden=self.have_solution()
                disabled=!self.props.enable_solve
                onclick=self.link.callback(|_| SolutionMsg::Solve)
                class=classes!("fa", "fa-play", "w3-theme-l5", "w3-round-large") />
              <button
                title="Reset"
                hidden=!self.have_solution()
                disabled=!self.have_solution()
                onclick=self.link.callback(|_| SolutionMsg::Reset)
                class=classes!("fa", "fa-undo", "w3-theme-l5", "w3-round-large") />
            </div>
            <p>{ self.props.solution.to_string() }</p>
          </div>
        }
    }
}
