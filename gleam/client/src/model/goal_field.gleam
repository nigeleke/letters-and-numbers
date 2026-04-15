import gleam/option.{type Option, None, Some}

import domain/goal.{type Goal, type ValidationError}

pub type GoalField {
  Unset
  Defined(raw: String, validated: Result(Goal, ValidationError))
}

pub fn new() -> GoalField {
  from_string("")
}

pub fn from_string(value: String) -> GoalField {
  case value == "" {
    True -> Unset
    False -> Defined(value, goal.from_string(value))
  }
}

pub fn from_goal(value: Goal) -> GoalField {
  Defined(goal.to_string(value), Ok(value))
}

pub fn update_field(_: GoalField, value: String) -> GoalField {
  from_string(value)
}

pub fn error_text(self: GoalField) -> Option(String) {
  case self {
    Unset -> None
    Defined(_, Ok(_)) -> None
    Defined(_, Error(error)) -> Some(goal.error_to_string(error))
  }
}
