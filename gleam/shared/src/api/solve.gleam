import domain/problem.{type Problem}
import gleam/json

pub fn endpoint() -> String {
  "/api/solve"
}

pub fn request_body(problem: Problem) -> json.Json {
  problem |> problem.to_json
}
