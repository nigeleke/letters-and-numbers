use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Number {
  Valid(isize),
  Invalid,
}

impl Number {
  pub fn new() -> Self {
    Number::Invalid
  }

  pub fn is_valid(&self) -> bool {
    matches!(*self, Number::Valid(_))
  }
}
