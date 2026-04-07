import core/resolver
import gleam/list
import gleeunit/should

pub type Defect {
  Defect(numbers: List(Int), goal: Int)
}

pub fn a_resolver_returns_a_none_result_for_an_impossible_goal_test() {
  let results = resolver.find_solutions([1, 2, 3, 4, 5, 6], 999)
  list.length(results)
  |> should.equal(0)
}

// pub fn a_resolver_returns_valid_expressions_for_a_simple_possible_goal_test() {
//   let results = resolver.find_solutions([1, 2, 3, 4, 5, 100], 100)
//   assert !list.is_empty(results)
// }

// pub fn a_resolver_returns_valid_expressions_for_a_complex_possible_goal_test() {
//   let results = resolver.find_solutions([1, 2, 3, 4, 5, 6], 720)
//   assert !list.is_empty(results)
// }

pub fn a_resolver_returns_valid_expressions_for_defects_test() {
  let defects = [
    Defect([75, 3, 6, 5, 5, 1], 559),
    // Defect([5, 8, 8, 2, 100, 50], 543),
  ]

  defects
  |> list.each(fn(defect) {
    let results = resolver.find_solutions(defect.numbers, defect.goal)
    assert !list.is_empty(results)
  })
}
