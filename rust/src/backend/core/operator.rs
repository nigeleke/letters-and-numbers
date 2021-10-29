#[derive(Debug, Clone, Copy)]
pub enum Operator {
  Plus,
  Minus,
  Times,
  Divides,
}

impl std::fmt::Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let op = match self {
      Operator::Plus => "+",
      Operator::Minus => "-",
      Operator::Times => "*",
      Operator::Divides => "/",
    };
    write!(f, "{}", op)
  }
}

impl Operator {
  pub fn apply(&self, a: isize, b: isize) -> Option<isize> {
    match self {
      Operator::Plus => {
        if a <= b {
          Some(a.saturating_add(b))
        } else {
          None
        }
      }
      Operator::Minus => {
        if a > b {
          Some(a.saturating_sub(b))
        } else {
          None
        }
      }
      Operator::Times => {
        if a != 1 && b != 1 && a <= b {
          Some(a.saturating_mul(b))
        } else {
          None
        }
      }
      Operator::Divides => {
        if b != 0 && b != 1 && a % b == 0 {
          Some(a / b)
        } else {
          None
        }
      }
    }
  }

  pub fn values() -> Vec<Operator> {
    vec![
      Operator::Plus,
      Operator::Minus,
      Operator::Times,
      Operator::Divides,
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn operator_to_string_shows_symbol() {
    assert_eq!(format!("{}", Operator::Plus), "+");
    assert_eq!(format!("{}", Operator::Minus), "-");
    assert_eq!(format!("{}", Operator::Times), "*");
    assert_eq!(format!("{}", Operator::Divides), "/");
  }

  #[quickcheck_macros::quickcheck]
  fn plus_operator_returns_value_when_first_operand_is_less_than_or_equal_to_second(
    a: isize,
    b: isize,
  ) -> bool {
    let result = Operator::Plus.apply(a, b);
    let expected_result = if a <= b {
      Some(a.saturating_add(b))
    } else {
      None
    };
    expected_result == result
  }

  #[quickcheck_macros::quickcheck]
  fn minus_operator_returns_value_when_first_operand_is_greater_than_second(
    a: isize,
    b: isize,
  ) -> bool {
    let result = Operator::Minus.apply(a, b);
    let expected_result = if a > b {
      Some(a.saturating_sub(b))
    } else {
      None
    };
    expected_result == result
  }

  #[quickcheck_macros::quickcheck]
  fn times_operator_returns_value_when_both_operands_are_not_identity_and_first_operand_is_less_than_equal_second(
    a: isize,
    b: isize,
  ) -> bool {
    let result = Operator::Times.apply(a, b);
    let expected_result = if a != 1 && b != 1 && a <= b {
      Some(a.saturating_mul(b))
    } else {
      None
    };
    expected_result == result
  }

  #[quickcheck_macros::quickcheck]
  fn divides_operator_returns_value_when_denominator_not_zero_or_identity_and_result_is_whole(
    a: isize,
    b: isize,
  ) -> bool {
    let result = Operator::Divides.apply(a, b);
    let expected_result = if b != 0 && b != 1 && a % b == 0 {
      Some(a / b)
    } else {
      None
    };
    expected_result == result
  }
}
