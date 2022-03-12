use super::operator::*;

use std::rc::*;

#[derive(Clone, Debug)]
pub enum Expression {
  Operand(isize),
  Operation(Rc<Expression>, Operator, Rc<Expression>),
}

impl Expression {
  fn operand(value: isize) -> Expression {
    Expression::Operand(value)
  }

  fn operation(left: Rc<Expression>, operator: Operator, right: Rc<Expression>) -> Expression {
    Expression::Operation(left, operator, right)
  }

  //   fn result(&self) -> Result {
  //     let maybe_value = self.value();
  //     match self {
  //       Expression::Operand(v) => Result::new(maybe_value, 1, &v.to_string()),
  //       Expression::Operation(left_result, operator, right_result) => Result::new(
  //         maybe_value,
  //         left_result.node_count + right_result.node_count,
  //         &match maybe_value {
  //           None => "".to_string(),
  //           Some(_) => format!(
  //             "({} {} {})",
  //             left_result.description, operator, right_result.description
  //           ),
  //         },
  //       ),
  //     }
  //   }

  pub fn from(numbers: &[isize]) -> Vec<Expression> {
    let size = numbers.len();
    if size <= 1 {
      numbers
        .iter()
        .map(|x| Expression::operand(*x))
        .collect::<Vec<Expression>>()
    } else {
      let operands = numbers.iter().map(|x| Expression::operand(*x));

      fn results_for(maybe_numbers: Option<&[isize]>) -> Vec<Expression> {
        let ns = Vec::from(maybe_numbers.unwrap_or_else(|| &[]));
        Expression::from(&ns)
      }

      let mut operations = Vec::new();
      for n in 1..size {
        for left_result in results_for(numbers.get(0..n)) {
          let left_rc = Rc::new(left_result);
          for right_result in results_for(numbers.get(n..size)) {
            let right_rc = Rc::new(right_result);
            for operator in Operator::values() {
              let operation = Expression::operation(left_rc.clone(), operator, right_rc.clone());
              operations.push(operation);
            }
          }
        }
      }

      operands
        .into_iter()
        .chain(operations.into_iter())
        .collect::<Vec<Expression>>()
    }
  }

  pub fn value(&self) -> Option<isize> {
    match self {
      Expression::Operand(v) => Some(*v),
      Expression::Operation(left, operator, right) => {
        let left_value = left.value();
        let right_value = right.value();
        match (left_value, right_value) {
          (Some(a), Some(b)) => operator.apply(a, b),
          (_, _) => None,
        }
      }
    }
  }

  pub fn description(&self) -> String {
    format!("{}", self)
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
  fn operand_always_has_a_value(v: isize) -> bool {
    let operand = Expression::Operand(v);
    operand.value() == Some(v)
  }

  #[quickcheck_macros::quickcheck]
  fn operation_returns_the_same_value_as_the_operator_applied_to_the_operands(
    a: isize,
    b: isize,
  ) -> bool {
    let operand_a = Expression::Operand(a);
    let operand_a_rc = Rc::new(operand_a);
    let operand_b = Expression::Operand(b);
    let operand_b_rc = Rc::new(operand_b);
    for operator in Operator::values() {
      let expression = Expression::Operation(operand_a_rc.clone(), operator, operand_b_rc.clone());
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
    assert_eq!(results.get(0).expect("Expected 4").value(), Some(4));
    assert_eq!(results.get(1).expect("Expected 2").value(), Some(2));
    assert_eq!(results.get(2).expect("Expected ![4+2]").value(), None);
    assert_eq!(results.get(3).expect("Expected 4-2").value(), Some(2));
    assert_eq!(results.get(4).expect("Expected ![4*2]").value(), None);
    assert_eq!(results.get(5).expect("Expected 4/2").value(), Some(2));
    assert!(results.get(6).is_none())
  }

  #[test]
  fn multiple_numbers_returns_expression_descriptions() {
    let numbers = vec![4, 2];
    let results = Expression::from(&numbers);
    assert_eq!(results.len(), 6);
    assert_eq!(results.get(0).expect("4").description(), "4");
    assert_eq!(results.get(1).expect("2").description(), "2");
    assert_eq!(results.get(2).expect("(4 + 2)").description(), "(4 + 2)");
    assert_eq!(results.get(3).expect("(4 - 2)").description(), "(4 - 2)");
    assert_eq!(results.get(4).expect("(4 * 2)").description(), "(4 * 2)");
    assert_eq!(results.get(5).expect("(4 / 2)").description(), "(4 / 2)");
    assert!(results.get(6).is_none())
  }
}
