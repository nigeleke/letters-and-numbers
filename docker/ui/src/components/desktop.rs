use yew::prelude::*;

use crate::model::goal::*;
use crate::model::numbers::*;

use super::goal_view::*;
use super::numbers_view::*;

use std::ops::Deref;

// use yew::{function_component, html, Html, Properties, Callback};
//
// #[derive(Properties, PartialEq)]
// pub struct Props {
//   pub on_name_entry: Callback<String>,
// }
//
// #[function_component]
// fn HelloWorld(props: &Props) -> Html {
//
//   props.on_name_entry.emit(String::from("Bob"));
//
//   html! { "Hello" }
// }
//
// // Then supply the prop
// #[function_component]
// fn App() -> Html {
//   let on_name_entry: Callback<String> = Callback::from(move |name: String| {
//     let greeting = format!("Hey, {}!", name);
//     // web_sys::console::log_1(&greeting.into()); // if uncommented will print
//   });
//
//   html! {<HelloWorld {on_name_entry} />}
// }


#[function_component(Desktop)]
pub fn desktop() -> Html {
  let goal_state = use_state(|| Goal::new());
  let goal = goal_state.deref().clone();
  let on_goal_change = Callback::from(move |updated_goal: Goal| {
    goal_state.set(updated_goal.clone());
  });

  let numbers_state = use_state(|| Numbers::new());
  let numbers = numbers_state.deref().clone();
  let on_numbers_change = Callback::from(move |updated_numbers: Numbers| {
    numbers_state.set(updated_numbers.clone());
  });

  html! {
    <div class="desktop">
      <GoalView enabled=true value={goal} on_change={on_goal_change} />
      <NumbersView enabled=true value={numbers} on_change={on_numbers_change} />
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
