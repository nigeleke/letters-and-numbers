use crate::core::resolver::*;
use crate::ui::goal::*;
use crate::ui::numbers::*;
use crate::ui::solution::*;

use yew::prelude::*;

#[derive(Debug)]
pub enum DesktopMsg {
  GoalUpdated(GoalValue),
  NumbersUpdated(NumbersValue),
  Action(SolutionMsg),
  Solve,
}

pub struct Desktop {
  link: ComponentLink<Self>,
  numbers: NumbersValue,
  goal: GoalValue,
  goal_revealed: bool,
  solving: bool,
  solution: String,
}

impl Desktop {
  fn enable_solve(&self) -> bool {
    self.goal.is_valid() && self.numbers.is_valid() &&!self.solving && self.solution.is_empty()
  }

  fn solve(numbers: &NumbersValue, goal: &GoalValue) -> String {
    let solutions = match (numbers, goal) {
      (NumbersValue::Valid(i1, i2, i3, i4, i5, i6), GoalValue::Auto(target)) => {
        Resolver::find_solutions(&[*i1, *i2, *i3, *i4, *i5, *i6], *target)
      }
      (NumbersValue::Valid(i1, i2, i3, i4, i5, i6), GoalValue::Manual(target)) => {
        Resolver::find_solutions(&[*i1, *i2, *i3, *i4, *i5, *i6], *target)
      }
      (_, _) => vec![], // Unreachable
    };

    if !solutions.is_empty() {
      solutions[0].to_string()
    } else {
      "No solution".to_string()
    }
  }
}

impl Component for Desktop {
  type Message = DesktopMsg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Desktop {
      link,
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
      }
      DesktopMsg::NumbersUpdated(numbers) => {
        self.numbers = numbers;
      }
      DesktopMsg::Action(SolutionMsg::Solve) => {
        self.solving = true;
        self.goal_revealed = true;
        self.link.send_message(DesktopMsg::Solve);
      }
      DesktopMsg::Solve => {
        self.solution = Desktop::solve(&self.numbers, &self.goal);
        self.solving = false;
      }
      DesktopMsg::Action(SolutionMsg::Reset) => {
        self.numbers = NumbersValue::Unset;
        self.goal = Goal::random_auto();
        self.goal_revealed = false;
        self.solution = "".to_string();
      }
    };
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
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
