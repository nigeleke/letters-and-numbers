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
  fn solve(numbers: NumbersValue, goal: GoalValue) -> String {
    let (real_numbers, target) = match (numbers, goal) {
      (Some(numbers), GoalValue::Manual(Some(target))) => (numbers, target),
      (Some(numbers), GoalValue::Auto(target)) => (numbers, target),
      (_, _) => ((0, 0, 0, 0, 0, 0), 0), // Unreachable
    };

    fn resolve(ns: (i32, i32, i32, i32, i32, i32), target: i32) -> String {
      let numbers_array = [ns.0, ns.1, ns.2, ns.3, ns.4, ns.5];
      let solutions = Resolver::find_solutions(numbers_array.as_ref(), target);
      if solutions.is_empty() {
        "No solution".to_string()
      } else {
        solutions[0].to_string()
      }
    }

    resolve(real_numbers, target)
  }
}

impl Component for Desktop {
  type Message = DesktopMsg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Desktop {
      link,
      numbers: None,
      goal: Goal::random_auto(),
      goal_revealed: false,
      solving: false,
      solution: "".to_string(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    log::info!("Desktop::Update {:?}", msg);
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
        self.solution = Desktop::solve(self.numbers, self.goal);
      }
      DesktopMsg::Action(SolutionMsg::Reset) => {
        self.numbers = None;
        self.goal = Goal::random_auto();
        self.goal_revealed = false;
        self.solving = false;
        self.solution = "".to_string();
      }
    };
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    log::info!("Desktop::change {:?}", props);
    true
  }

  fn view(&self) -> Html {
    html! {
      <div class=classes!("w3-center")>
        <Goal value=self.goal active=!self.solving revealed=self.goal_revealed on_update=self.link.callback(DesktopMsg::GoalUpdated) />
        <Numbers value=self.numbers active=!self.solving on_update=self.link.callback(DesktopMsg::NumbersUpdated) />
        <Solution numbers=self.numbers goal=self.goal solution=self.solution.to_string() active=!self.solving on_action=self.link.callback(DesktopMsg::Action) />
      </div>
    }
  }
}
