import gleam/dynamic/decode
import gleam/int
import gleam/json
import gleam/list

pub opaque type Number {
  Number(Int)
}

pub opaque type ValidationError {
  NotNumeric
  InvalidValue
}

pub fn from_string(value: String) -> Result(Number, ValidationError) {
  case int.parse(value) {
    Ok(i) -> from_int(i)
    Error(_) -> Error(NotNumeric)
  }
}

fn from_int(value: Int) -> Result(Number, ValidationError) {
  case is_small_int(value) || is_large_int(value) {
    True -> Ok(Number(value))
    False -> Error(InvalidValue)
  }
}

pub fn is_small(value: Number) -> Bool {
  let Number(value) = value
  is_small_int(value)
}

fn is_small_int(value: Int) -> Bool {
  [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] |> list.contains(value)
}

pub fn is_large(value: Number) -> Bool {
  let Number(value) = value
  is_large_int(value)
}

fn is_large_int(value: Int) -> Bool {
  [25, 50, 75, 100] |> list.contains(value)
}

pub fn to_string(n: Number) -> String {
  let Number(i) = n
  i |> int.to_string
}

pub fn error_to_string(error: ValidationError) -> String {
  case error {
    NotNumeric -> "not numeric"
    InvalidValue -> "must be in range 1-10, 25, 50, 75, 100"
  }
}

pub fn to_json(number: Number) -> json.Json {
  let Number(value) = number
  json.object([#("value", json.int(value))])
}

pub fn to_json_string(number: Number) -> String {
  number
  |> to_json
  |> json.to_string
}

pub fn decoder() -> decode.Decoder(Number) {
  use value <- decode.field("value", decode.int)
  case from_int(value) {
    Ok(number) -> decode.success(number)
    Error(error) -> decode.failure(Number(value), error_to_string(error))
  }
}

pub fn from_json_string(
  json_string: String,
) -> Result(Number, json.DecodeError) {
  json.parse(from: json_string, using: decoder())
}

pub fn to_int(number: Number) -> Int {
  let Number(value) = number
  value
}
