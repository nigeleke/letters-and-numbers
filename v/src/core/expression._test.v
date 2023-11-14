module core

fn test_operand_always_has_a_value() {
  operand := as_operand(42);
  assert operand.value()? == 42
}

fn test_operation_returns_the_same_value_as_the_operator_applied_to_the_operands() {
  a := as_operand(10)
  b := as_operand(5)

  for operator in operator_values() {
    operation := as_operation(a, operator, b)
    operation_value := operation.value() or { 0 }
    a_value := a.value() or { 0 }
    b_value := b.value() or { 0 }
    operator_value := operator.apply(a_value, b_value) or { 0 }
    assert operation_value == operator_value
  }
}

fn test_single_number_returns_single_result() {
  numbers := [1]
  results := expressions_from(numbers)
  assert results.len == 1
}


fn is_empty(maybeValue ?int) bool {
	return match maybeValue {
		none { true }
		else { false }
	}
}


fn test_multiple_numbers_returns_valid_result_operations() {
  mut numbers := [4, 2]
  results := expressions_from(numbers)
  for result in results {
    println('${result}')
  }
  assert results.len == 2
  assert results[0].value()? == 2
  assert results[1].value()? == 2
}

fn test_multiple_numbers_returns_valid_result_descriptions() {
  numbers := [4, 2]
  results := expressions_from(numbers)
  assert results.len == 2
  assert results[0].description() == '(4 - 2)'
  assert results[1].description() == '(4 / 2)'
}
