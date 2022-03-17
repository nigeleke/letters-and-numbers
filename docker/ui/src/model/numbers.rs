// use super::number::*;
//
// use serde::*;
//
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub enum Numbers {
//   Valid(Vec<Number>),
//   Invalid(Vec<Number>),
// }
//
// impl Numbers {
//   pub fn new() -> Self {
//     Numbers::Invalid(vec![Number::new(); 6])
//   }
//
//   pub fn is_valid(&self) -> bool {
//     match self {
//       Numbers::Valid(_) => true,
//       Numbers::Invalid(_) => false,
//     }
//   }
// }
