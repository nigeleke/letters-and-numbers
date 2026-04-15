import gleam/dynamic
import gleam/dynamic/decode
import gleam/json.{type Json}

import domain/goal.{type Goal}
import domain/numbers.{type Numbers}

pub type Problem {
  Problem(numbers: Numbers, goal: Goal)
}

pub fn to_json(problem: Problem) -> Json {
  let Problem(numbers, goal) = problem
  json.object([
    #("numbers", numbers.to_json(numbers)),
    #("goal", goal.to_json(goal)),
  ])
}

pub fn to_json_string(problem: Problem) -> String {
  problem
  |> to_json
  |> json.to_string
}

pub fn decoder() -> decode.Decoder(Problem) {
  use numbers <- decode.field("numbers", numbers.decoder())
  use goal <- decode.field("goal", goal.decoder())
  decode.success(Problem(numbers, goal))
}

pub fn from_json(
  json: dynamic.Dynamic,
) -> Result(Problem, List(decode.DecodeError)) {
  decode.run(json, decoder())
}

pub fn from_json_string(
  json_string: String,
) -> Result(Problem, json.DecodeError) {
  json.parse(from: json_string, using: decoder())
}
