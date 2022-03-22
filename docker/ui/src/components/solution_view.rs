use yew::prelude::*;

use crate::model::solution::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
  pub value: Solution,
}

#[function_component(SolutionView)]
pub fn solution_view(props: &Props) -> Html {
  let value = props.value.clone();
  let inner_html = match value {
    Solution::Unsolved => html!{},
    Solution::Solving => html!{ <p>{"Solving"}</p> },
    Solution::Solved(s) => html!{ <p>{s.to_string()}</p> },
    Solution::Reset => html!{},
  };

  html! {
    <div class="solution">{ inner_html }</div>
  }
}
