import gleam/list
import gleam/result
import gleeunit/should

import core/expression.{type Expression, Operand, Operation}
import core/operator.{type Operator, Divide, Minus, Plus, Times}

fn as_operand(value: Int) -> Expression {
  Operand(value)
}

fn as_operation(
  left: Expression,
  operator: Operator,
  right: Expression,
) -> Expression {
  Operation(left:, operator:, right:)
}

fn operator_values() -> List(Operator) {
  [Plus, Minus, Times, Divide]
}

pub fn operand_always_has_a_value_test() {
  let operand = as_operand(42)

  expression.evaluate(operand)
  |> should.equal(Ok(42))
}

pub fn operation_returns_the_same_value_as_the_operator_applied_to_the_operands_test() {
  let a = as_operand(10)
  let b = as_operand(5)

  operator_values()
  |> list.each(fn(op) {
    let operation = as_operation(a, op, b)

    let assert Ok(operation_value) = expression.evaluate(operation)
    let assert Ok(a_value) = expression.evaluate(a)
    let assert Ok(b_value) = expression.evaluate(b)

    let expected = case op {
      Plus -> a_value + b_value
      Minus ->
        case a_value > b_value {
          True -> a_value - b_value
          False -> 0
        }
      Times -> a_value * b_value
      Divide ->
        case b_value != 0 && a_value % b_value == 0 {
          True -> a_value / b_value
          False -> 0
        }
    }

    operation_value
    |> should.equal(expected)
  })
}

pub fn single_number_returns_single_result_test() {
  let numbers = [1]
  let results = expression.from_ints(numbers)

  results
  |> list.length()
  |> should.equal(1)
}

pub fn multiple_numbers_returns_valid_result_operations_test() {
  let numbers = [4, 2]
  let results = expression.from_ints(numbers)

  results
  |> list.length()
  |> should.equal(4)

  results
  |> list.first
  |> result.try(expression.evaluate)
  |> should.equal(Ok(6))

  results
  |> list.last
  |> result.try(expression.evaluate)
  |> should.equal(Ok(2))
}

pub fn multiple_numbers_returns_valid_result_descriptions_test() {
  let numbers = [4, 2]
  let results = expression.from_ints(numbers)

  results
  |> list.length()
  |> should.equal(4)

  results
  |> list.first
  |> result.map(expression.description)
  |> should.equal(Ok("(4 + 2)"))

  results
  |> list.last
  |> result.map(expression.description)
  |> should.equal(Ok("(4 / 2)"))
}
