use crate::resolver::*;

use serde::*;
use yew::worker::*;

use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  Solve(Vec<i32>, i32),
}

pub struct ResolverService {
  link: AgentLink<ResolverService>,
  subscribers: HashSet<HandlerId>,
}

impl Agent for ResolverService {
  type Reach = Context<Self>;
  type Message = ();
  type Input = Request;
  type Output = String;

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
        let solutions = Resolver::find_solutions(&numbers, goal);
        let no_solution = "No Solution".to_string();
        let solution = solutions.get(0).unwrap_or(&no_solution);
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
