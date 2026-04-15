import gleam/dynamic/decode
import gleam/json.{type Json}
import gleam/option.{type Option, None, Some}

pub opaque type Solution {
  Solution(Option(String))
}

pub fn from_option_string(value: Option(String)) -> Solution {
  Solution(value)
}

pub fn to_string(solution: Solution) -> String {
  let Solution(solution) = solution
  case solution {
    Some(value) -> value
    None -> "No solution"
  }
}

pub fn to_json(solution: Solution) -> Json {
  let Solution(solution) = solution
  json.object([#("value", json.nullable(solution, json.string))])
}

pub fn to_json_string(solution: Solution) -> String {
  solution
  |> to_json
  |> json.to_string
}

pub fn decoder() -> decode.Decoder(Solution) {
  use value <- decode.field("value", decode.optional(decode.string))
  decode.success(Solution(value))
}

pub fn from_json_string(
  json_string: String,
) -> Result(Solution, json.DecodeError) {
  json.parse(from: json_string, using: decoder())
}
