use yew::prelude::*;

use gloo_net::http::*;
use gloo_net::*;
use wasm_bindgen_futures::spawn_local;

use crate::config::default_config::*;

use crate::model::goal::*;
use crate::model::numbers::*;
use crate::model::solution::*;

use super::action::*;

pub struct SolveAction;

impl SolveAction {
  pub fn using(
    goal: &Goal,
    numbers: &Numbers,
    solution: &Solution,
    on_change: Callback<Solution>,
  ) -> Action {
    let goal = goal.clone();
    let numbers = numbers.clone();

    let unsolved = matches!(solution, Solution::Unsolved);
    let enabled = unsolved && goal.is_valid() && numbers.is_valid();
    let solved = matches!(solution, Solution::Solved(_));
    let visible = !solved;

    async fn make_solve_request_result(goal: Goal, numbers: Numbers) -> Result<String, Error> {
      let values = numbers.clone().values;
      let url = format!(
        "{}/api/v1/solve?n1={}&n2={}&n3={}&n4={}&n5={}&n6={}&goal={}",
        DefaultConfig::default().api_url,
        values[0],
        values[1],
        values[2],
        values[3],
        values[4],
        values[5],
        goal
      );
      Request::get(&url).send().await?.text().await
    }

    async fn make_solve_request(goal: Goal, numbers: Numbers, on_change: Callback<Solution>) {
      on_change.emit(Solution::Solving);
      let response = make_solve_request_result(goal, numbers);
      match response.await {
        Ok(r) => on_change.emit(Solution::Solved(r)),
        Err(_e) => on_change.emit(Solution::Solved(
          "Failed to get solution - try later.".to_owned(),
        )),
      };
    }

    let on_execute = Callback::from(move |_| {
      let goal = goal.clone();
      let numbers = numbers.clone();
      let on_change = on_change.clone();
      spawn_local(make_solve_request(goal, numbers, on_change));
    });

    Action::new("Solve", "fas fa-play", enabled, visible, on_execute)
  }
}
