use super::number::*;

use itertools::*;
use serde::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Numbers {
  pub values: Vec<Number>,
}

impl Numbers {
  pub fn from(values: &[Number]) -> Self {
    Self {
      values: Vec::from(values),
    }
  }

  pub fn undefined() -> Self {
    let values = vec![Number::undefined(); 6];
    Self { values }
  }

  pub fn is_individually_valid(&self) -> bool {
    let valid_numbers: Vec<&Number> = self.values.iter().filter(|n| n.is_valid()).collect();
    valid_numbers.iter().len() == 6
  }

  pub fn is_valid(&self) -> bool {
    let valid_numbers_len = self.values.iter().filter(|n| n.is_valid()).count();
    let mut counts = self.values.iter().counts_by(|n| n.value);

    counts.retain(|i, count| {
      if (1..=10).contains(i) {
        *count > 2
      } else if vec![25, 50, 75, 100].contains(i) {
        *count > 1
      } else {
        true
      }
    });

    valid_numbers_len == 6 && counts.is_empty()
  }

  pub fn number(&self, i: usize) -> Number {
    self.values[i]
  }

  pub fn updated(&self, i: usize, number: Number) -> Numbers {
    let mut values = self.values.clone();
    values[i] = number;
    Numbers::from(&values)
  }
}

/*
fn validate_group(ns: &[Number]) -> bool {
  fn value_of(n: &Number) -> isize {
    match *n {
      Number::Valid(i) => i,
      Number::Invalid => 0
    }
  }

  let number_values: Vec<isize> = ns.iter().map(|n| value_of(n)).collect();
  let mut counts = number_values.iter().counts_by(|n| n);

  counts.retain(|i, count| {
    if (1..=10).contains(*i) { *count > 2 }
    else if vec![25, 50, 75, 100].contains(*i) { *count > 1 }
    else { true }
  });

  counts.is_empty()
}
 */
