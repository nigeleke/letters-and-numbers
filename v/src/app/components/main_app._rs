use super::goal_view::*;
use super::icon_button::*;
use super::numbers_view::*;
use super::solution_view::*;

use crate::api::resolver_service::*;
use crate::frontend::actions::action::*;
use crate::shared::goal::*;
use crate::shared::numbers::*;
use crate::shared::solution::*;

use yew::agent::*;
use yew::prelude::*;

pub struct MainApp {
  link: ComponentLink<Self>,
  numbers: Numbers,
  goal: Goal,
  solution: Solution,
  resolver_service: Dispatcher<ResolverService>,
  _resolver_response: Box<dyn Bridge<ResolverService>>,
  solve_action: Action,
  reset_action: Action,
}

#[derive(Debug)]
pub enum Msg {
  GoalUpdated(Goal),
  NumbersUpdated(Numbers),
  SolveAction(),
  Solved(Solution),
  ResetAction(),
}

impl Component for MainApp {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let on_reset = link.callback(|_| Msg::ResetAction());
    let on_solve = link.callback(|_| Msg::SolveAction());
    let resolver_callback = link.callback(Msg::Solved);
    Self {
      link,
      numbers: Numbers::new(),
      goal: Goal::new(),
      solution: Solution::new(),
      resolver_service: ResolverService::dispatcher(),
      _resolver_response: ResolverService::bridge(resolver_callback),
      solve_action: Action::solve_action(on_solve),
      reset_action: Action::reset_action(on_reset),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    let is_valid_input = |goal: &Goal, numbers: &Numbers| goal.is_valid() && numbers.is_valid();
    match msg {
      Msg::GoalUpdated(goal) => {
        self.solve_action.enabled = is_valid_input(&goal, &self.numbers);
        self.goal = goal;
      }
      Msg::NumbersUpdated(numbers) => {
        self.solve_action.enabled = is_valid_input(&self.goal, &numbers);
        self.numbers = numbers;
      }
      Msg::SolveAction() => {
        let current_goal = self.goal.clone();
        self.goal = match current_goal {
          Goal::ValidAuto(goal) => Goal::ValidManual(goal),
          _ => current_goal,
        };
        self.solve_action.visible = false;
        self.reset_action.visible = true;
        self
          .resolver_service
          .send(Request::Solve(self.numbers.clone(), self.goal.clone()));
      }
      Msg::Solved(solution) => {
        self.solution = solution;
        self.reset_action.enabled = true;
      }
      Msg::ResetAction() => {
        self.goal = Goal::new();
        self.numbers = Numbers::new();
        self.solution = Solution::new();
        self.solve_action.visible = true;
        self.reset_action.visible = false;
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let on_change_goal = self.link.callback(Msg::GoalUpdated);
    let on_change_numbers = self.link.callback(Msg::NumbersUpdated);
    html! {
      <div class="main-app">
        <GoalView value=self.goal.clone() on_change=on_change_goal />
        <NumbersView value=self.numbers.clone() on_change=on_change_numbers />
        <span>
          <IconButton action=self.solve_action.clone() />
          <IconButton action=self.reset_action.clone() />
        </span>
        <SolutionView value=self.solution.clone() />
      </div>
    }
  }
}
