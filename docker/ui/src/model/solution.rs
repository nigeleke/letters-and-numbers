use serde::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Solution {
  Unsolved,
  Solving,
  Solved(String),
  Reset,
}

impl Solution {
  pub fn unsolved() -> Self {
    Solution::Unsolved
  }

  // pub fn solved(maybe_solution: Option<&String>) -> Self {
  //   let no_solution = "No Solution".to_string();
  //   let solution = maybe_solution.unwrap_or(&no_solution);
  //   Solution {
  //     value: solution.to_owned(),
  //   }
  // }
}
