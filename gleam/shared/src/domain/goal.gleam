import gleam/dynamic/decode
import gleam/int
import gleam/json

pub opaque type Goal {
  Goal(Int)
}

pub opaque type ValidationError {
  NotNumeric
  InvalidValue
}

pub fn error_to_string(error: ValidationError) -> String {
  case error {
    NotNumeric -> "not numeric"
    InvalidValue -> "must be in range 100-999"
  }
}

pub fn from_string(value: String) -> Result(Goal, ValidationError) {
  case int.parse(value) {
    Ok(n) -> from_int(n)
    Error(_) -> Error(NotNumeric)
  }
}

const lower_bound = 100

const upper_bound = 999

pub fn from_int(value: Int) -> Result(Goal, ValidationError) {
  case lower_bound <= value && value <= upper_bound {
    True -> Ok(Goal(value))
    False -> Error(InvalidValue)
  }
}

pub fn to_string(goal: Goal) -> String {
  let Goal(n) = goal
  n |> int.to_string
}

pub fn to_json(goal: Goal) -> json.Json {
  let Goal(value) = goal
  json.object([#("value", json.int(value))])
}

pub fn to_json_string(goal: Goal) -> String {
  goal
  |> to_json
  |> json.to_string
}

pub fn decoder() -> decode.Decoder(Goal) {
  use value <- decode.field("value", decode.int)
  case from_int(value) {
    Ok(goal) -> decode.success(goal)
    Error(error) -> decode.failure(Goal(value), error_to_string(error))
  }
}

pub fn from_json_string(json_string: String) -> Result(Goal, json.DecodeError) {
  json.parse(from: json_string, using: decoder())
}

pub fn to_int(goal: Goal) -> Int {
  let Goal(value) = goal
  value
}
