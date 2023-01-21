use rand::distributions::{Distribution, Uniform};
use serde::*;
use std::fmt::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Goal {
  is_auto: bool,
  value: isize
}

impl Goal {
  pub fn auto() -> Self {
    let mut rng = rand::thread_rng();
    let target = Uniform::from(100..=999);
    let value = target.sample(&mut rng);
    Self {
      is_auto: true,
      value
    }
  }

  pub fn from(value: isize) -> Self {
    Self {
      is_auto: false,
      value
    }
  }

  pub fn undefined() -> Self {
    Self {
      is_auto: false,
      value: 0
    }
  }

  pub fn is_valid(&self) -> bool {
    (100..=999).contains(&self.value)
  }
}

impl Display for Goal {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.value)
  }
}