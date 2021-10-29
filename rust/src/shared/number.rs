use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Number {
  Unset,
  Valid(isize),
  Invalid,
}

impl Number {
  pub fn new() -> Self {
    Number::Unset
  }

  pub fn is_valid(&self) -> bool {
    matches!(*self, Number::Valid(_))
  }
}
