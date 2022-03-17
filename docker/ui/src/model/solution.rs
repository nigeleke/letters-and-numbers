// use serde::*;
//
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Solution {
//   value: String,
// }
//
// impl Solution {
//   pub fn new() -> Self {
//     Solution {
//       value: "".to_owned(),
//     }
//   }
//
//   pub fn solved(maybe_solution: Option<&String>) -> Self {
//     let no_solution = "No Solution".to_string();
//     let solution = maybe_solution.unwrap_or(&no_solution);
//     Solution {
//       value: solution.to_owned(),
//     }
//   }
// }
//
// impl std::fmt::Display for Solution {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     write!(f, "{}", self.value)
//   }
// }
