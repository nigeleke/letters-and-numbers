import gleam/list
import gleam/option.{type Option, None, Some}

import domain/numbers.{type Numbers, type ValidationError}
import model/number_field.{type NumberField}

pub type NumbersFields {
  NumbersFields(
    raw: List(NumberField),
    validated: Result(Numbers, ValidationError),
  )
}

pub fn new() -> NumbersFields {
  let fields = list.repeat(number_field.new(), 6)
  from_list(fields)
}

fn from_list(fields: List(NumberField)) -> NumbersFields {
  let numbers = fields |> list.filter_map(number_field.to_number)
  NumbersFields(fields, numbers.from_list(numbers))
}

pub fn update_field(
  fields: NumbersFields,
  index: Int,
  value: String,
) -> NumbersFields {
  let NumbersFields(raw_list, _) = fields

  let updated_raw =
    raw_list
    |> list.index_map(fn(field, i) {
      case i == index {
        True -> number_field.from_string(value)
        False -> field
      }
    })

  from_list(updated_raw)
}

pub fn error_text(self: NumbersFields) -> Option(String) {
  case self.validated {
    Ok(_) -> None
    Error(error) -> Some(numbers.error_to_string(error))
  }
}
