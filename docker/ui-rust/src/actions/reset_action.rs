use yew::prelude::*;

use crate::model::goal::*;
use crate::model::numbers::*;
use crate::model::solution::*;

use super::action::*;

pub struct ResetAction;

impl ResetAction {
  pub fn using(
    solution: &Solution,
    on_solution_change: Callback<Solution>,
    on_goal_change: Callback<Goal>,
    on_numbers_change: Callback<Numbers>,
  ) -> Action {
    let solution = solution.clone();
    let solved = matches!(solution, Solution::Solved(_));

    let enabled = solved;
    let visible = solved;

    let on_execute = Callback::from(move |_| {
      on_goal_change.emit(Goal::auto());
      on_numbers_change.emit(Numbers::undefined());
      on_solution_change.emit(Solution::Reset);
      on_solution_change.emit(Solution::Unsolved);
    });

    Action::new("Reset", "fas fa-undo", enabled, visible, on_execute)
  }
}
