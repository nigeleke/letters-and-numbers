module core

fn test_a_resolver_returns_a_none_result_for_an_impossible_goal() {
  results := find_solutions([1, 2, 3, 4, 5, 6], 999)
  assert results.len == 0
}

fn test_a_resolver_returns_valid_expressions_for_a_simple_possible_goal() {
  results := find_solutions([1, 2, 3, 4, 5, 100], 100)
  assert results.len > 0
}

fn test_a_resolver_returns_valid_expressions_for_a_complex_possible_goal() {
  results := find_solutions([1, 2, 3, 4, 5, 6], 720)
  assert results.len > 0
}

struct Defect {
  numbers []int
  goal int
}

fn test_a_resolver_returns_valid_expressions_for_defects() {
  defects := [
    Defect { [75, 3, 6, 5, 5, 1], 559 },
    Defect { [5, 8, 8, 2, 100, 50], 543 }
  ]
  for defect in defects {
    results := find_solutions(defect.numbers, defect.goal)
    assert results.len > 0
  }
}

