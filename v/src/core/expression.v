module core

struct Operand {
  value int
}

struct Operation {
  left Expression
  operator Operator
  right Expression
}

type Expression = Operand | Operation

pub fn as_operand(value int) Expression {
  return Operand { value }
}

pub fn as_operation(left Expression, operator Operator, right Expression) Expression {
  return Operation { left, operator, right }
}

fn (operand Operand) value() ?int { return operand.value }

fn (operation Operation) value() ?int {
  left_value := operation.left.value()
  right_value := operation.right.value()
  return match left_value {
    none { none }
    else {
      match right_value {
        none { none }
        else { operation.operator.apply(left_value?, right_value?) }
      }
    }
  }
}

pub fn (self Expression) value() ?int {
  return match self {
    Operand { self.value() }
    Operation { self.value() }
  }
}

fn (operand Operand) description() string { return '${operand.value}' }

fn (operation Operation) description() string {
  return '(${operation.left.description()} ${operation.operator.to_string()} ${operation.right.description()})'
}

pub fn (self Expression) description() string {
  return match self {
    Operand { self.description() }
    Operation { self.description() }
  }
}

pub fn expressions_from(numbers []int) []Expression {
  mut results := []Expression{}
  if numbers.len == 1 {
    results << as_operand(numbers[0])
  } else {
    for i in 1..numbers.len {
      lefts := expressions_from(numbers[..i])
      rights := expressions_from(numbers[i..])
      for left in lefts {
        for right in rights {
          for operator in operator_values() {
            operation := as_operation(left, operator, right)
            match operation.value() {
              none {}
              else { results << operation }
            }
          }
        }
      }
    }
  }
  return results
}


//   pub fn description(&self) -> String {
//     format!("{}", self)
//   }
// }

// impl std::fmt::Display for Expression {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     match self {
//       Expression::Operand(v) => write!(f, "{}", v),
//       Expression::Operation(left, operator, right) => {
//         write!(f, "({} {} {})", left, operator, right)
//       }
//     }
//   }
// }
