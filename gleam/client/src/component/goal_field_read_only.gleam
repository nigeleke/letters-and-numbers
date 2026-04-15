import lustre/attribute.{class}
import lustre/element.{type Element}
import lustre/element/html.{div, text}

import domain/goal.{type Goal, type ValidationError}
import model/goal_field.{type GoalField, Defined, Unset}

pub fn view(field: GoalField) -> Element(msg) {
  let #(raw, validated) = case field {
    // Need valid value; random will be generated if / when required
    Unset -> #("", goal.from_int(100))
    Defined(value, validated) -> #(value, validated)
  }

  div([class("goal-field")], [
    div([class(input_class(validated))], [text(raw)]),
    error_view(validated),
  ])
}

fn input_class(validated: Result(Goal, ValidationError)) -> String {
  case validated {
    Ok(_) -> "valid"
    Error(_) -> "invalid"
  }
}

fn error_view(validated: Result(Goal, ValidationError)) -> Element(msg) {
  case validated {
    Ok(_) -> element.fragment([])
    Error(error) ->
      div([class("goal-field__error")], [
        text(goal.error_to_string(error)),
      ])
  }
}
