import gleam/int
import gleam/list
import gleam/result

import core/operator.{type Operator, Divide, Minus, Plus, Times}

pub type Expression {
  Operand(Int)
  Operation(left: Expression, operator: Operator, right: Expression)
}

pub fn from_ints(numbers: List(Int)) -> List(Expression) {
  let operands = list.map(numbers, Operand)

  case operands {
    [] -> []
    _ -> generate_operations(operands)
  }
}

fn generate_operations(items: List(Expression)) -> List(Expression) {
  let n = list.length(items)

  case items {
    [single] -> [single]

    _ -> {
      int.range(1, n, [], fn(acc, split) {
        let left_items = list.take(items, split)
        let right_items = list.drop(items, split)

        let lefts = generate_operations(left_items)
        let rights = generate_operations(right_items)

        let build_expressions = fn(left, right) {
          [Plus, Minus, Times, Divide]
          |> list.map(fn(op) { Operation(left, op, right) })
          |> list.filter_map(fn(e) {
            case evaluate(e) {
              Ok(_) -> Ok(e)
              Error(_) -> Error(Nil)
            }
          })
        }

        list.append(
          acc,
          lefts
            |> list.flat_map(fn(left) {
              rights |> list.flat_map(build_expressions(left, _))
            }),
        )
      })
    }
  }
}

pub fn evaluate(expression: Expression) -> Result(Int, Nil) {
  case expression {
    Operand(value) -> Ok(value)

    Operation(left, operator, right) -> {
      use left_val <- result.try(evaluate(left))
      use right_val <- result.try(evaluate(right))

      case operator {
        Plus -> Ok(left_val + right_val)
        Minus ->
          case left_val - right_val {
            res if res > 0 -> Ok(res)
            _ -> Error(Nil)
          }
        Times -> Ok(left_val * right_val)
        Divide ->
          case right_val != 0 && left_val % right_val == 0 {
            True -> Ok(left_val / right_val)
            False -> Error(Nil)
          }
      }
    }
  }
}

pub fn description(expression: Expression) -> String {
  let wrap = fn(left: String, op: String, right: String) -> String {
    "(" <> left <> " " <> op <> " " <> right <> ")"
  }

  case expression {
    Operand(value) -> int.to_string(value)
    Operation(left, operator, right) ->
      wrap(description(left), operator.to_string(operator), description(right))
  }
}
