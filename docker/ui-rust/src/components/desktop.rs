use style4rs::style;
use yew::prelude::*;

use crate::actions::action::*;
use crate::actions::reset_action::*;
use crate::actions::solve_action::*;

use crate::model::goal::*;
use crate::model::numbers::*;
use crate::model::solution::*;

use super::goal_view::*;
use super::icon_button::*;
use super::numbers_view::*;
use super::solution_view::*;

use std::ops::Deref;

#[function_component(Desktop)]
pub fn desktop() -> Html {
  let class = style! {
    div {
      display: flex;
      flex-flow: column;
      justify-content: space-between;
      align-items: center;
    }
  };

  let solution_state = use_state(Solution::unsolved);
  let solution = solution_state.deref().clone();

  let on_solution_change = Callback::from(move |updated_solution: Solution| {
    solution_state.set(updated_solution);
  });

  let goal_state = use_state(Goal::auto);
  let goal = goal_state.deref().clone();

  let on_goal_change = Callback::from(move |updated_goal: Goal| {
    goal_state.set(updated_goal);
  });

  let numbers_state = use_state(Numbers::undefined);
  let numbers = numbers_state.deref().clone();

  let on_numbers_change = Callback::from(move |updated_numbers: Numbers| {
    let numbers_state = numbers_state.clone();
    numbers_state.set(updated_numbers);
  });

  let solve_action_ref = use_mut_ref(Action::undefined);
  let updated_solve_action =
    SolveAction::using(&goal, &numbers, &solution, on_solution_change.clone());
  solve_action_ref.replace(updated_solve_action);
  let solve_action = solve_action_ref.deref().clone().into_inner();

  let reset_action_ref = use_mut_ref(Action::undefined);
  let updated_reset_action = ResetAction::using(
    &solution,
    on_solution_change,
    on_goal_change.clone(),
    on_numbers_change.clone(),
  );
  reset_action_ref.replace(updated_reset_action);
  let reset_action = reset_action_ref.deref().clone().into_inner();

  html! {
    <div class={class}>
      <NumbersView value={numbers} solution={solution.clone()} on_change={on_numbers_change} />
      <GoalView value={goal} solution={solution.clone()} on_change={on_goal_change} />
      <span>
        <IconButton action={solve_action} />
        <IconButton action={reset_action} />
      </span>
      <SolutionView value={solution} />
    </div>
  }
}
