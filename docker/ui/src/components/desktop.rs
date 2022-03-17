use yew::prelude::*;

use crate::goal_view::*;
use crate::numbers_view::*;

#[function_component(Desktop)]
pub fn desktop() -> Html {
    html! {
      <div class="desktop">
        <GoalView enabled=true valid=true />
        <NumbersView enabled=true valid=true />
      </div>
    }
}

// pub struct MainApp {
//   link: ComponentLink<Self>,
//   numbers: Numbers,
//   goal: Goal,
//   solution: Solution,
//   solve_action: Action,
//   reset_action: Action,
// }
//
// #[derive(Debug)]
// pub enum Msg {
//   GoalUpdated(Goal),
//   NumbersUpdated(Numbers),
//   SolveAction(),
//   Solved(Solution),
//   ResetAction(),
// }

// #[function_component(MainApp)]
// pub fn main_app() -> Html {
// type Message = Msg;
// type Properties = ();
//
// fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
//   let on_reset = link.callback(|_| Msg::ResetAction());
//   let on_solve = link.callback(|_| Msg::SolveAction());
//   let resolver_callback = link.callback(Msg::Solved);
//   Self {
//     link,
//     numbers: Numbers::new(),
//     goal: Goal::new(),
//     solution: Solution::new(),
//     solve_action: Action::solve_action(on_solve),
//     reset_action: Action::reset_action(on_reset),
//   }
// }
//
// fn update(&mut self, msg: Self::Message) -> ShouldRender {
//   let is_valid_input = |goal: &Goal, numbers: &Numbers| goal.is_valid() && numbers.is_valid();
//   match msg {
//     Msg::GoalUpdated(goal) => {
//       self.solve_action.enabled = is_valid_input(&goal, &self.numbers);
//       self.goal = goal;
//     }
//     Msg::NumbersUpdated(numbers) => {
//       self.solve_action.enabled = is_valid_input(&self.goal, &numbers);
//       self.numbers = numbers;
//     }
//     Msg::SolveAction() => {
//       let current_goal = self.goal.clone();
//       self.goal = match current_goal {
//         Goal::ValidAuto(goal) => Goal::ValidManual(goal),
//         _ => current_goal,
//       };
//       self.solve_action.visible = false;
//       self.reset_action.visible = true;
//     }
//     Msg::Solved(solution) => {
//       self.solution = solution;
//       self.reset_action.enabled = true;
//     }
//     Msg::ResetAction() => {
//       self.goal = Goal::new();
//       self.numbers = Numbers::new();
//       self.solution = Solution::new();
//       self.solve_action.visible = true;
//       self.reset_action.visible = false;
//     }
//   }
//   true
// }
//
// fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//   false
// }
//
//   let on_change_goal = self.link.callback(Msg::GoalUpdated);
//   let on_change_numbers = self.link.callback(Msg::NumbersUpdated);

//   html! {
//     <div class="main-app">
//       <GoalView value=self.goal.clone() on_change=on_change_goal />
//       <NumbersView value=self.numbers.clone() on_change=on_change_numbers />
//       <span>
//         <IconButton action=self.solve_action.clone() />
//         <IconButton action=self.reset_action.clone() />
//       </span>
//       <SolutionView value=self.solution.clone() />
//     </div>
// }

//   html! {
//       <>
//       <div class="main-app">
//       <p>{"Main App"}</p>
//       </div>
//       </>
//   // }
// }
