use crate::backend::core::resolver::*;
use crate::shared::goal::*;
use crate::shared::number::*;
use crate::shared::numbers::*;
use crate::shared::solution::*;

use serde::*;
use yew::worker::*;

use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  Solve(Numbers, Goal),
}

#[derive(Debug)]
pub struct ResolverService {
  link: AgentLink<ResolverService>,
  subscribers: HashSet<HandlerId>,
}

impl Agent for ResolverService {
  // TODO: Change to Public<Self> when supported...
  type Reach = Context<Self>;
  type Message = ();
  type Input = Request;
  type Output = Solution;

  fn create(link: AgentLink<Self>) -> Self {
    Self {
      link,
      subscribers: HashSet::new(),
    }
  }

  fn update(&mut self, _msg: Self::Message) {}

  fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
    match msg {
      Request::Solve(numbers, goal) => {
        let (raw_numbers, raw_goal) = match (numbers, goal) {
          (Numbers::Valid(raw_numbers), Goal::ValidAuto(raw_goal)) => (raw_numbers, raw_goal),
          (Numbers::Valid(raw_numbers), Goal::ValidManual(raw_goal)) => (raw_numbers, raw_goal),
          (_, _) => (vec![], 0), // Unreachable
        };
        let mut raw_inumbers: Vec<isize> = vec![0; 6];
        for i in 0..6 {
          raw_inumbers[i] = match raw_numbers[i] {
            Number::Valid(n) => n,
            _ => 0, // Unreachable
          }
        }
        let solutions = Resolver::find_solutions(&raw_inumbers, raw_goal);
        let solution = Solution::solved(solutions.get(0));
        for id in self.subscribers.iter() {
          if id.is_respondable() {
            self.link.respond(*id, solution.to_owned());
          }
        }
      }
    }
  }

  fn connected(&mut self, id: HandlerId) {
    self.subscribers.insert(id);
  }

  fn disconnected(&mut self, id: HandlerId) {
    self.subscribers.remove(&id);
  }
}
