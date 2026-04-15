import lustre/attribute.{class, id, value}
import lustre/element.{type Element}
import lustre/element/html.{div, input}
import lustre/event.{on_input}

import domain/number.{type Number, type ValidationError}
import model/number_field.{type NumberField, NumberField}

pub fn view(
  ident: String,
  field: NumberField,
  on_change: fn(String) -> msg,
) -> Element(msg) {
  let NumberField(raw, validated) = field

  div([class("number-field")], [
    input([
      class(input_class(validated)),
      id(ident),
      value(raw),
      on_input(on_change),
    ]),
  ])
}

fn input_class(validated: Result(Number, ValidationError)) -> String {
  case validated {
    Ok(_) -> "valid"
    Error(_) -> "invalid"
  }
}
