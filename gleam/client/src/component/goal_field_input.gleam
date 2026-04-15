import lustre/attribute.{class, id, placeholder, value}
import lustre/element
import lustre/element/html.{div, input, text}
import lustre/event.{on_input}

import domain/goal.{type Goal, type ValidationError}
import model/goal_field.{type GoalField, Defined, Unset}

pub fn view(
  ident: String,
  field: GoalField,
  on_change: fn(String) -> msg,
) -> element.Element(msg) {
  let #(raw, validated) = case field {
    // Need valid value; random will be generated if / when required
    Unset -> #("", goal.from_int(100))
    Defined(value, validated) -> #(value, validated)
  }

  div([class("goal-field")], [
    input([
      class(input_class(validated)),
      id(ident),
      placeholder("- Auto -"),
      value(raw),
      on_input(on_change),
    ]),

    error_view(validated),
  ])
}

fn input_class(validated: Result(Goal, ValidationError)) -> String {
  case validated {
    Ok(_) -> "valid"
    Error(_) -> "invalid"
  }
}

fn error_view(
  validated: Result(Goal, ValidationError),
) -> element.Element(msg) {
  case validated {
    Ok(_) -> element.fragment([])
    Error(error) ->
      div([class("goal-field__error")], [
        text(goal.error_to_string(error)),
      ])
  }
}
