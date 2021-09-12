use crate::core::expression::*;
use crate::core::result::*;

use comp::iter;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::IntoIterator;
use std::iter::Iterator;

pub struct Resolver;

impl Resolver {
  pub fn find_solutions(operands: &[i32], goal: i32) -> Vec<String> {
    let results: HashSet<String> = iter! {
      let n <- 1..6;
      let combination <- operands.iter().combinations(n);
      let len = combination.len();
      let permutation <- combination.into_iter().permutations(len);
      let results = permutation.expressions();
      let result <- results;
      if result.value == Some(goal);
      result.to_string()
    }
    .collect::<HashSet<String>>();

    results.into_iter().collect::<Vec<String>>()
  }
}

trait ExpressionsExt {
  fn expressions(&self) -> std::vec::Vec<Result>;
}

impl ExpressionsExt for Vec<&i32> {
  fn expressions(&self) -> std::vec::Vec<Result> {
    // TODO: Find a way to map from Vec<&i32> to Vec<i32> without the mutable.
    let mut deref_vec = Vec::new();
    for i in self {
      deref_vec.push(**i);
    }
    Expression::from(&deref_vec)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_resolver_returns_a_none_result_for_an_impossible_goal() {
    let results = Resolver::find_solutions(&vec![1, 2, 3, 4, 5, 6], 999);
    assert_eq!(results.len(), 0);
  }

  #[test]
  fn a_resolver_returns_valid_expressions_for_a_simple_possible_goal() {
    let results = Resolver::find_solutions(&vec![1, 2, 3, 4, 5, 100], 100);
    assert!(results.len() > 0);
  }

  #[test]
  fn a_resolver_returns_valid_expressions_for_a_complex_possible_goal() {
    let results = Resolver::find_solutions(&vec![1, 2, 3, 4, 5, 6], 720);
    assert!(results.len() > 0);
  }
}
