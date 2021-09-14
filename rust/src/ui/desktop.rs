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
  numbers: Option<NumbersValue>,
  goal: Option<GoalValue>,
  goal_revealed: bool,
  solving: bool,
  solution: String,
}

impl Desktop {
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

    if solutions.len() > 0 {
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
      numbers: None,
      goal: None,
      goal_revealed: false,
      solving: false,
      solution: "".to_string(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    log::info!("Desktop::update {:?}", msg);
    match msg {
      DesktopMsg::GoalUpdated(goal) => {
        self.goal = Some(goal);
      }
      DesktopMsg::NumbersUpdated(numbers) => {
        self.numbers = Some(numbers);
      }
      DesktopMsg::Action(SolutionMsg::Solve) => {
        self.solving = true;
        self.goal_revealed = true;
        self.link.send_message(DesktopMsg::Solve);
      }
      DesktopMsg::Solve => {
        let numbers = self
          .numbers
          .as_ref()
          .unwrap_or(&NumbersValue::Valid(0, 0, 0, 0, 0, 0));
        let goal = self.goal.as_ref().unwrap_or(&GoalValue::Auto(0));
        self.solution = Desktop::solve(numbers, goal);
        self.solving = false;
      }
      DesktopMsg::Action(SolutionMsg::Reset) => {
        log::info!("Reset: ");
        self.numbers = None;
        self.goal = None;
        self.goal_revealed = false;
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
        <Goal active=!self.solving revealed=self.goal_revealed on_update=self.link.callback(DesktopMsg::GoalUpdated) />
        <Numbers active=!self.solving on_update=self.link.callback(DesktopMsg::NumbersUpdated) />
        <Solution solution=self.solution.to_string() active=!self.solving on_action=self.link.callback(DesktopMsg::Action) />
      </div>
    }
  }
}
