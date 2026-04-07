import core/expression
import gleam/int
import gleam/list

pub fn find_solutions(operands: List(Int), goal: Int) -> List(String) {
  let is_goal = fn(e) {
    case expression.evaluate(e) {
      Ok(value) if value == goal -> Ok(e)
      _ -> Error(Nil)
    }
  }

  let n = list.length(operands)

  int.range(1, n + 1, [], fn(acc, i) {
    list.combinations(operands, i)
    |> list.flat_map(list.permutations)
    |> list.flat_map(expression.from_ints)
    |> list.filter_map(is_goal)
    |> list.map(expression.description)
    |> list.append(acc, _)
  })
}
