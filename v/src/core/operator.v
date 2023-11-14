module core

pub enum Operator {
  plus
  minus
  times
  divides
}

pub fn (self Operator) to_string() string {
  return match self {
    .plus { "+" }
    .minus { "-" }
    .times { "*" }
    .divides { "/" }
  }
}

pub fn (self Operator) apply(a int, b int) ?int {
  return match self {
    .plus { if a <= b { a + b } else { none }}
    .minus { if a > b { a - b } else { none } }
    .times { if a != 1 && b != 1 && a <= b { a * b } else { none } }
    .divides { if b != 0 && b != 1 && a % b == 0 { a / b } else { none } }
  }
}

pub fn operator_values() []Operator {
  return [
    Operator.plus,
    Operator.minus,
    Operator.times,
    Operator.divides,
  ]
}

