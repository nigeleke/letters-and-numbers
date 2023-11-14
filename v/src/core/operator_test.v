module core

fn test_operator_to_string_shows_symbol() {
	assert Operator.plus.to_string() == "+"
	assert Operator.minus.to_string() == "-"
	assert Operator.times.to_string() == "*"
	assert Operator.divides.to_string() == "/"
}

fn is_empty(maybeValue ?int) bool {
	return match maybeValue {
		none { true }
		else { false }
	}
}

fn test_plus_operator_returns_value_when_first_operand_is_less_than_or_equal_to_second() {
	assert Operator.plus.apply(2, 2)? == 4
	assert Operator.plus.apply(2, 4)? == 6
	assert is_empty(Operator.plus.apply(4, 2))
}

fn test_minus_operator_returns_value_when_first_operand_is_greater_than_second() {
	assert is_empty(Operator.minus.apply(2, 2))
	assert is_empty(Operator.minus.apply(2, 4))
	assert Operator.minus.apply(4, 2)? == 2
}

fn test_times_operator_returns_value_when_both_operands_are_not_identity_and_first_operand_is_less_than_equal_second() {
	assert is_empty(Operator.times.apply(1, 1))
	assert is_empty(Operator.times.apply(1, 2))
	assert is_empty(Operator.times.apply(2, 1))
	assert Operator.times.apply(2, 2)? == 4
	assert Operator.times.apply(2, 4)? == 8
	assert is_empty(Operator.times.apply(4, 2))
}

fn test_divides_operator_returns_value_when_denominator_not_zero_or_identity_and_result_is_whole() {
	assert is_empty(Operator.divides.apply(2, 0))
	assert is_empty(Operator.divides.apply(2, 1))
	assert is_empty(Operator.divides.apply(3, 2))
	assert Operator.divides.apply(6, 2)? == 3
}
