module core

import arrays

fn permutations(ns []int) [][]int {
  mut results := [][]int{}
  if ns.len == 0 {
  } else if ns.len == 1 {
    results << ns
  } else {
    for i in 0 .. ns.len {
      x := ns[i]
      xs := arrays.append(ns[..i], ns[i+1..])
      for p in permutations(xs) {
        results << arrays.append([x], p)
      }
    }
  }

  return results
}

pub fn combinations(ns []int, n int) [][]int {
	mut results := [][]int{}
  for i in 0..ns.len {
    if n == 1 {
      results << [ns[i]]
    } else {
      for c in combinations(ns[i+1..], n-1) {
        results << arrays.append([ns[i]], c)
      }
    }
  }
	return results
}


pub fn find_solutions(operands []int, goal int) []string {
  mut solutions := []string{}

  for i in 0..operands.len {
    for combination in combinations(operands, i+1) {
      for permutation in permutations(combination) {
        for expression in expressions_from(permutation) {
          expression_value := expression.value() or { 0 }
          if expression_value == goal {
            solutions << expression.description()
          }
        }
      }
    }
  }

  return solutions
}

// trait ExpressionsExt {
//   fn expressions(&self) -> std::vec::Vec<Expression>;
// }

// impl ExpressionsExt for Vec<int> {
//   fn expressions(&self) -> std::vec::Vec<Expression> {
//     Expression::from(self)
//   }
// }
