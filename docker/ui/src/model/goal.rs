// use rand::distributions::{Distribution, Uniform};
// use serde::*;
//
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub enum Goal {
//   ValidAuto(isize),
//   ValidManual(isize),
//   Invalid,
// }
//
// impl Goal {
//   pub fn new() -> Self {
//     Goal::random_auto()
//   }
//
//   fn random_auto() -> Self {
//     let mut rng = rand::thread_rng();
//     let target = Uniform::from(100..=999);
//     Goal::ValidAuto(target.sample(&mut rng))
//   }
//
//   pub fn is_valid(&self) -> bool {
//     *self != Goal::Invalid
//   }
// }
