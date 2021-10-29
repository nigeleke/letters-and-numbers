use super::expression::*;

use comp::iter;
use itertools::Itertools;
use std::iter::IntoIterator;
use std::iter::Iterator;

pub struct Resolver;

impl Resolver {
  pub fn find_solutions(operands: &[isize], goal: isize) -> Vec<String> {
    let solutions = iter! {
      let n <- 1..=6;
      let permutation <- Vec::from(operands).into_iter().permutations(n);
      let expressions = permutation.expressions();
      let expression <- expressions;
      let value = expression.value();
      if value == Some(goal);
      expression
    };

    solutions
      .into_iter()
      .map(|s| s.description())
      .collect::<Vec<String>>()
  }
}

trait ExpressionsExt {
  fn expressions(&self) -> std::vec::Vec<Expression>;
}

impl ExpressionsExt for Vec<isize> {
  fn expressions(&self) -> std::vec::Vec<Expression> {
    Expression::from(self)
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

  #[test]
  fn a_resolver_returns_valid_expressions_for_defects() {
    let defects = vec![
      (vec![75, 3, 6, 5, 5, 1], 559),
      (vec![5, 8, 8, 2, 100, 50], 543),
    ];
    for defect in defects {
      let (numbers, goal) = defect;
      let results = Resolver::find_solutions(&numbers, goal);
      assert!(results.len() > 0);
    }
  }
}
