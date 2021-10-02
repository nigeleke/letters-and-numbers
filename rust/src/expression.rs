use crate::operator::*;
use crate::result::*;

#[derive(Debug)]
pub enum Expression {
  Operand(i32),
  Operation(Result, Operator, Result),
}

impl Expression {
  fn operand(value: i32) -> Expression {
    Expression::Operand(value)
  }

  fn operation(left_result: &Result, operator: Operator, right_result: &Result) -> Expression {
    Expression::Operation(
      (*left_result).to_owned(),
      operator,
      (*right_result).to_owned(),
    )
  }

  fn result(&self) -> Result {
    let maybe_value = self.value();
    match self {
      Expression::Operand(v) => Result::new(maybe_value, 1, &v.to_string()),
      Expression::Operation(left_result, operator, right_result) => Result::new(
        maybe_value,
        left_result.node_count + right_result.node_count,
        &match maybe_value {
          None => "".to_string(),
          Some(_) => format!(
            "({} {} {})",
            left_result.description, operator, right_result.description
          ),
        },
      ),
    }
  }

  pub fn from(numbers: &[i32]) -> Vec<Result> {
    let size = numbers.len();
    if size <= 1 {
      numbers
        .iter()
        .map(|x| Expression::operand(*x))
        .map(|x| x.result())
        .collect::<Vec<Result>>()
    } else {
      let operands = numbers
        .iter()
        .map(|x| Expression::operand(*x))
        .map(|x| x.result());

      fn results_for(maybe_numbers: Option<&[i32]>) -> Vec<Result> {
        let ns = maybe_numbers.unwrap_or_else(|| &[]);
        Expression::from(&Vec::from(ns))
      }

      // TODO: Find out why, when using comp and iter!, I get the 'must move' warning for left & right results.
      //       Thereby removing the need for the mutable.
      let mut operations = Vec::new();
      for n in 1..size {
        for left_result in results_for(numbers.get(0..n)) {
          for right_result in results_for(numbers.get(n..size)) {
            for operator in Operator::values() {
              let operation = Expression::operation(&left_result, operator, &right_result);
              operations.push(operation.result());
            }
          }
        }
      }

      operands
        .into_iter()
        .chain(operations.into_iter())
        .collect::<Vec<Result>>()
    }
  }
}

trait Value {
  fn value(&self) -> Option<i32>;
}

impl Value for Expression {
  fn value(&self) -> Option<i32> {
    match self {
      Expression::Operand(v) => Some(*v),
      Expression::Operation(left, operator, right) => match (left.value, right.value) {
        (Some(a), Some(b)) => operator.apply(a, b),
        (_, _) => None,
      },
    }
  }
}

impl std::fmt::Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Expression::Operand(v) => write!(f, "{}", v),
      Expression::Operation(left, operator, right) => {
        write!(f, "({} {} {})", left, operator, right)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[quickcheck_macros::quickcheck]
  fn operand_always_has_a_value(v: i32) -> bool {
    let operand = Expression::Operand(v);
    operand.value() == Some(v)
  }

  #[quickcheck_macros::quickcheck]
  fn operation_returns_the_same_value_as_the_operator_applied_to_the_operands(
    a: i32,
    b: i32,
  ) -> bool {
    for operator in Operator::values() {
      let operand_a = Expression::Operand(a);
      let operand_b = Expression::Operand(b);
      let expression = Expression::Operation(operand_a.result(), operator, operand_b.result());
      assert_eq!(expression.value(), operator.apply(a, b))
    }
    true
  }

  #[test]
  fn single_number_returns_single_result() {
    let numbers = vec![1];
    let results = Expression::from(&numbers);
    assert_eq!(results.len(), 1)
  }

  #[test]
  fn multiple_numbers_returns_result_per_operand_and_per_operation() {
    let numbers = vec![4, 2];
    let results = Expression::from(&numbers);
    assert_eq!(results.len(), 6);
    assert_eq!(results.get(0).expect("Expected 4").value, Some(4));
    assert_eq!(results.get(1).expect("Expected 2").value, Some(2));
    assert_eq!(results.get(2).expect("Expected ![4+2]").value, None);
    assert_eq!(results.get(3).expect("Expected 4-2").value, Some(2));
    assert_eq!(results.get(4).expect("Expected ![4*2]").value, None);
    assert_eq!(results.get(5).expect("Expected 4/2").value, Some(2));
    assert!(results.get(6).is_none())
  }
}
