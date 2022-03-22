use serde::*;
use std::fmt::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Number {
  pub value: isize
}

impl Number {
  pub fn from(value: isize) -> Self {
    Self {
      value
    }
  }

  pub fn undefined() -> Self {
    Self {
      value: 0
    }
  }

  pub fn is_valid(&self) -> bool {
    (1..=10).contains(&self.value) ||
      vec![25, 50, 75, 100].contains(&self.value)
  }
}

impl Display for Number {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.value)
  }
}