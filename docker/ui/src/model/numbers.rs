use super::number::*;

use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Numbers {
  Valid(Number, Number, Number, Number, Number, Number),
  Invalid(Number, Number, Number, Number, Number, Number),
}

impl Numbers {
  pub fn new() -> Self {
    Numbers::Invalid(
      Number::new(),
      Number::new(),
      Number::new(),
      Number::new(),
      Number::new(),
      Number::new())
  }

  pub fn is_valid(&self) -> bool {
    match self {
      Numbers::Valid(_, _, _, _, _, _) => true,
      Numbers::Invalid(_, _, _, _, _, _) => false,
    }
  }
}
