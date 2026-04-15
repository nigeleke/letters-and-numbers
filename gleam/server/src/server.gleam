import envoy
import gleam/erlang/process
import gleam/http.{Get, Post}
import gleam/int
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import lustre/attribute.{href, id, rel, src, type_}
import lustre/element
import lustre/element/html.{body, div, head, html, link, script, title}
import mist
import wisp.{type Request, type Response}
import wisp/wisp_mist

import core/resolver
import domain/goal
import domain/numbers
import domain/problem
import domain/solution

pub fn main() {
  wisp.configure_logger()
  let secret_key_base = wisp.random_string(64)

  let assert Ok(priv_directory) = wisp.priv_directory("server")
  let static_directory = priv_directory <> "/static"

  let host = case envoy.get("HOST") {
    Ok(host) -> host
    Error(_) -> "127.0.0.1"
  }

  let port = case envoy.get("PORT") {
    Ok(port) ->
      case int.parse(port) {
        Ok(i) -> i
        Error(_) -> 8080
      }
    Error(_) -> 8080
  }

  let assert Ok(_) =
    handle_request(static_directory, _)
    |> wisp_mist.handler(secret_key_base)
    |> mist.new
    |> mist.bind(host)
    |> mist.port(port)
    |> mist.start

  process.sleep_forever()
}

fn app_middleware(
  req: Request,
  static_directory: String,
  next: fn(Request) -> Response,
) -> Response {
  let req = wisp.method_override(req)
  use <- wisp.log_request(req)
  use <- wisp.rescue_crashes
  use req <- wisp.handle_head(req)
  use <- wisp.serve_static(req, under: "/static", from: static_directory)

  next(req)
}

fn handle_request(static_directory: String, req: Request) -> Response {
  use req <- app_middleware(req, static_directory)

  case req.method, wisp.path_segments(req) {
    Get, _ -> serve_index()
    Post, ["api", "solve"] -> solve(req)
    _, _ -> wisp.not_found()
  }
}

fn serve_index() -> Response {
  html([], [
    head([], [
      title([], "Letters and Numbers"),
      link([rel("stylesheet"), href("/static/app.css")]),
      link([rel("stylesheet"), href("/static/client.css")]),
      link([rel("stylesheet"), href("/static/goal_field.css")]),
      link([rel("stylesheet"), href("/static/number_field.css")]),
      link([rel("stylesheet"), href("/static/numbers_fields.css")]),
      link([rel("stylesheet"), href("/static/solution_field.css")]),
      script([type_("module"), src("/static/client.js")], ""),
    ]),
    body([], [div([id("app")], [])]),
  ])
  |> element.to_document_string
  |> wisp.html_response(200)
}

fn solve(req: Request) -> Response {
  use json <- wisp.require_json(req)

  let result = {
    use problem <- result.try(problem.from_json(json))

    let problem.Problem(numbers, goal) = problem
    let solution =
      resolver.find_solutions(numbers.to_int_list(numbers), goal.to_int(goal))
      |> list.first
      |> option.from_result
      |> solution.from_option_string

    Ok(solution)
  }

  case result {
    Ok(solution) ->
      wisp.ok()
      |> wisp.json_body(solution.to_json_string(solution))

    Error(msg) ->
      wisp.bad_request("Unexpected error")
      |> wisp.string_body(string.inspect(msg))
  }
}
