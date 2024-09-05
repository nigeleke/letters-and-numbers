use crate::domain::{Goal, Number, Numbers, Solution};

#[derive(Clone, Debug)]
pub enum GameState {
    Entering(Goal, Numbers),
    Solving(Goal, Numbers),
    Solved(Goal, Numbers, Solution),
}

impl GameState {
    pub fn goal(&self) -> Goal {
        match self {
            Self::Entering(goal, _) => goal.clone(),
            Self::Solving(goal, _) => goal.clone(),
            Self::Solved(goal, _, _) => goal.clone(),
        }
    }

    pub fn numbers(&self) -> Numbers {
        match self {
            Self::Entering(_, numbers) => numbers.clone(),
            Self::Solving(_, numbers) => numbers.clone(),
            Self::Solved(_, numbers, _) => numbers.clone(),
        }
    }

    pub fn update_number(&mut self, index: usize, number: &Number) {
        match self {
            Self::Entering(_, numbers) => numbers.update(index, number),
            _ => unreachable!(),
        }
    }

    pub fn update_goal(&mut self, new_goal: &Goal) {
        match self {
            Self::Entering(goal, _) => *goal = new_goal.clone(),
            _ => unreachable!(),
        }
    }

    pub fn reveal_goal(&mut self) {
        match self {
            Self::Entering(goal, numbers) => {
                goal.reveal();
                *self = Self::Solving(goal.clone(), numbers.clone());
            },
            _ => unreachable!()
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::Entering(Goal::default(), Numbers::default())
    }
}
