import gleam/dict
import gleam/dynamic/decode
import gleam/json.{type Json}
import gleam/list
import gleam/result

import domain/number.{type Number}

pub opaque type Numbers {
  Numbers(List(Number))
}

pub opaque type ValidationError {
  InvalidCount(Int)
  TooManyOccurrences
}

pub fn from_list(numbers: List(Number)) -> Result(Numbers, ValidationError) {
  use _ <- result.try(validate_length(numbers))
  use _ <- result.try(validate_smalls(numbers))
  use _ <- result.try(validate_larges(numbers))

  Ok(Numbers(numbers))
}

fn validate_length(numbers: List(Number)) -> Result(Nil, ValidationError) {
  case list.length(numbers) {
    6 -> Ok(Nil)
    len -> Error(InvalidCount(len))
  }
}

fn validate_smalls(numbers: List(Number)) -> Result(Nil, ValidationError) {
  let smalls = numbers |> list.filter(number.is_small)

  case smalls |> occurs_more_than(2) {
    True -> Error(TooManyOccurrences)
    False -> Ok(Nil)
  }
}

fn validate_larges(numbers: List(Number)) -> Result(Nil, ValidationError) {
  let larges = numbers |> list.filter(number.is_large)

  case larges |> occurs_more_than(1) {
    True -> Error(TooManyOccurrences)
    False -> Ok(Nil)
  }
}

fn occurs_more_than(items: List(Number), limit: Int) -> Bool {
  let count = fn(acc, item) {
    acc
    |> dict.insert(item, case dict.get(acc, item) {
      Ok(n) -> n + 1
      Error(_) -> 1
    })
  }

  let counts = items |> list.fold(dict.new(), count)

  counts
  |> dict.values()
  |> list.any(fn(c) { c > limit })
}

pub fn error_to_string(error: ValidationError) -> String {
  case error {
    InvalidCount(_) -> "numbers must in 1-10, 25, 50, 75 or 100"
    TooManyOccurrences -> "invalid duplications found"
  }
}

pub fn to_json(numbers: Numbers) -> Json {
  let Numbers(list) = numbers
  json.object([#("numbers", json.array(list, of: number.to_json))])
}

pub fn to_json_string(numbers: Numbers) -> String {
  numbers
  |> to_json
  |> json.to_string
}

pub fn decoder() -> decode.Decoder(Numbers) {
  use value <- decode.field("numbers", decode.list(number.decoder()))
  case from_list(value) {
    Ok(numbers) -> decode.success(numbers)
    Error(error) -> decode.failure(Numbers(value), error_to_string(error))
  }
}

pub fn from_json_string(
  json_string: String,
) -> Result(Numbers, json.DecodeError) {
  json.parse(from: json_string, using: decoder())
}

pub fn to_int_list(numbers: Numbers) -> List(Int) {
  let Numbers(numbers) = numbers
  numbers |> list.map(number.to_int)
}
