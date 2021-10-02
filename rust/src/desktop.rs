use crate::goal::*;
use crate::numbers::*;
use crate::resolver_service::*;
use crate::solution::*;

use yew::agent::*;
use yew::prelude::*;

#[derive(Debug)]
pub enum DesktopMsg {
  GoalUpdated(GoalValue),
  NumbersUpdated(NumbersValue),
  Action(SolutionMsg),
  Solve,
  Solved(String),
}

pub struct Desktop {
  link: ComponentLink<Self>,
  resolver: Dispatcher<ResolverService>,
  _resolution: Box<dyn Bridge<ResolverService>>,
  numbers: NumbersValue,
  goal: GoalValue,
  goal_revealed: bool,
  solving: bool,
  solution: String,
}

impl Desktop {
  fn enable_solve(&self) -> bool {
    self.goal.is_valid() && self.numbers.is_valid() && !self.solving && self.solution.is_empty()
  }

  fn unwrap(numbers: NumbersValue, goal: GoalValue) -> (Vec<i32>, i32) {
    match (numbers, goal) {
      (NumbersValue::Valid(i1, i2, i3, i4, i5, i6), GoalValue::Auto(target)) => {
        (vec![i1, i2, i3, i4, i5, i6], target)
      }
      (NumbersValue::Valid(i1, i2, i3, i4, i5, i6), GoalValue::Manual(target)) => {
        (vec![i1, i2, i3, i4, i5, i6], target)
      }
      (_, _) => (vec![], 0), // Unreachable
    }
  }
}

impl Component for Desktop {
  type Message = DesktopMsg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(DesktopMsg::Solved);
    Self {
      link,
      resolver: ResolverService::dispatcher(),
      _resolution: ResolverService::bridge(callback),
      numbers: NumbersValue::Unset,
      goal: Goal::random_auto(),
      goal_revealed: false,
      solving: false,
      solution: "".to_string(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      DesktopMsg::GoalUpdated(goal) => {
        self.goal = goal;
        true
      }
      DesktopMsg::NumbersUpdated(numbers) => {
        self.numbers = numbers;
        true
      }
      DesktopMsg::Action(SolutionMsg::Solve) => {
        self.solving = true;
        self.goal_revealed = true;
        self.link.send_message(DesktopMsg::Solve);
        true
      }
      DesktopMsg::Solve => {
        let (numbers, goal) = Desktop::unwrap(self.numbers, self.goal);
        self.resolver.send(Request::Solve(numbers, goal));
        false
      }
      DesktopMsg::Solved(solution) => {
        self.solution = solution;
        self.solving = false;
        true
      }
      DesktopMsg::Action(SolutionMsg::Reset) => {
        self.numbers = NumbersValue::Unset;
        self.goal = Goal::random_auto();
        self.goal_revealed = false;
        self.solution = "".to_string();
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div class=classes!("w3-center")>
        <Goal value=self.goal active=!self.solving revealed=self.goal_revealed on_update=self.link.callback(DesktopMsg::GoalUpdated) />
        <Numbers value=self.numbers active=!self.solving on_update=self.link.callback(DesktopMsg::NumbersUpdated) />
        <Solution enable_solve=self.enable_solve() solution=self.solution.to_string() on_action=self.link.callback(DesktopMsg::Action) />
      </div>
    }
  }
}
