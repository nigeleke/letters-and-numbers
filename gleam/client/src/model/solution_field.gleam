import gleam/int
import gleam/string
import rsvp

import domain/solution.{type Solution}

pub type SolutionField {
  Responded(Solution)
  Failed(String)
}

pub fn from_api_response(
  result: Result(Solution, rsvp.Error),
) -> SolutionField {
  case result {
    Ok(solution) -> Responded(solution)
    Error(error) -> Failed(error_to_string(error))
  }
}

fn error_to_string(error: rsvp.Error) -> String {
  case error {
    rsvp.BadBody -> "Bad response body"
    rsvp.BadUrl(url) -> "Invalid URL: " <> url
    rsvp.NetworkError -> "Network error (check your connection)"
    rsvp.HttpError(resp) ->
      "HTTP error "
      <> int.to_string(resp.status)
      <> " "
      <> string.inspect(resp.headers)
    rsvp.JsonError(decode_err) ->
      "Failed to decode JSON: " <> string.inspect(decode_err)
    rsvp.UnhandledResponse(resp) ->
      "Unexpected response: "
      <> int.to_string(resp.status)
      <> " ("
      <> string.inspect(resp.body)
      <> ")"
  }
}
