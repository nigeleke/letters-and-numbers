import gleam/int
import gleam/list
import lustre/attribute.{class}
import lustre/element
import lustre/element/html.{div, text}

import component/number_field_read_only
import domain/numbers.{type Numbers, type ValidationError}
import model/numbers_fields.{type NumbersFields}

pub fn view(fields: NumbersFields) -> element.Element(msg) {
  let id = fn(i: Int) -> String { "number-" <> int.to_string(i) }

  div([class("numbers-fields")], [
    div(
      [class("numbers-fields__numbers")],
      fields.raw
        |> list.index_map(fn(field, index) {
          number_field_read_only.view(id(index), field)
        }),
    ),
    error_view(fields.validated),
  ])
}

fn error_view(
  validated: Result(Numbers, ValidationError),
) -> element.Element(msg) {
  case validated {
    Ok(_) -> element.fragment([])
    Error(error) ->
      div([class("numbers-fields__error")], [
        text(numbers.error_to_string(error)),
      ])
  }
}
