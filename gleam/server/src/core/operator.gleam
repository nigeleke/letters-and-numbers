pub type Operator {
  Plus
  Minus
  Times
  Divide
}

pub fn to_string(operator: Operator) -> String {
  case operator {
    Plus -> "+"
    Minus -> "-"
    Times -> "*"
    Divide -> "/"
  }
}
