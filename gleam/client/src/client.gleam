import gleam/int
import lustre
import lustre/attribute.{class}
import lustre/effect.{type Effect}
import lustre/element.{type Element}
import lustre/element/html.{button, footer, h1, header, section, text}
import lustre/event.{on_click}
import rsvp

import api/solve
import component/goal_field_input
import component/goal_field_read_only
import component/numbers_fields_input
import component/numbers_fields_read_only
import component/solution_field_read_only
import domain/goal.{type Goal}
import domain/problem
import domain/solution.{type Solution}
import model/goal_field.{type GoalField, Defined, Unset}
import model/numbers_fields.{type NumbersFields}
import model/solution_field.{type SolutionField}

pub fn main() {
  let app = lustre.application(init, update, view)
  let assert Ok(_) = lustre.start(app, "#app", [])

  Nil
}

type Model {
  Setup(numbers: NumbersFields, goal: GoalField)
  Solving(numbers: NumbersFields, goal: GoalField)
  Solved(numbers: NumbersFields, goal: GoalField, solution: SolutionField)
}

fn init(_) -> #(Model, Effect(Msg)) {
  let numbers = numbers_fields.new()
  let goal = goal_field.new()
  let model = Setup(numbers:, goal:)

  #(model, effect.none())
}

type Msg {
  NumbersChanged(Int, String)
  PendingGoalChanged(String)
  SolutionRequested
  GoalDefined(Goal)
  SolutionReceived(Result(Solution, rsvp.Error))
  ResetRequested
}

fn update(model: Model, msg: Msg) -> #(Model, Effect(Msg)) {
  case msg {
    NumbersChanged(i, s) -> model |> update_number(i, s)
    PendingGoalChanged(s) -> model |> update_pending_goal(s)
    SolutionRequested -> model |> request_solution()
    GoalDefined(g) -> model |> update_goal(g)
    SolutionReceived(result) -> model |> update_solution(result)
    ResetRequested -> model |> reset_problem()
  }
}

fn update_number(
  model: Model,
  index: Int,
  value: String,
) -> #(Model, Effect(msg)) {
  case model {
    Setup(numbers, goal) -> {
      let updated_numbers = numbers |> numbers_fields.update_field(index, value)
      #(Setup(updated_numbers, goal), effect.none())
    }
    _ -> #(model, effect.none())
  }
}

fn update_pending_goal(model: Model, value: String) -> #(Model, Effect(Msg)) {
  case model {
    Setup(numbers, goal) -> {
      let updated_goal = goal |> goal_field.update_field(value)
      #(Setup(numbers, updated_goal), effect.none())
    }

    _ -> #(model, effect.none())
  }
}

fn request_solution(model: Model) -> #(Model, Effect(Msg)) {
  let get_random_goal = fn() {
    use dispatch <- effect.from
    let value = int.random(899) + 100
    case goal.from_int(value) {
      Ok(goal) -> dispatch(GoalDefined(goal))
      Error(_) -> Nil
    }
  }

  case model.goal {
    Unset -> #(model, get_random_goal())
    Defined(_, Ok(goal)) -> model |> update_goal(goal)
    _ -> #(model, effect.none())
  }
}

fn update_goal(model: Model, goal: Goal) -> #(Model, Effect(Msg)) {
  let updated_goal = goal_field.from_goal(goal)
  let updated_model = Solving(model.numbers, updated_goal)

  case model.numbers.validated {
    Ok(numbers) -> {
      let problem = problem.Problem(numbers, goal)

      let request =
        rsvp.post(
          solve.endpoint(),
          solve.request_body(problem),
          rsvp.expect_json(solution.decoder(), SolutionReceived),
        )

      #(updated_model, request)
    }
    _ -> #(updated_model, effect.none())
  }
}

fn update_solution(
  model: Model,
  result: Result(Solution, rsvp.Error),
) -> #(Model, Effect(Msg)) {
  let solution = solution_field.from_api_response(result)
  let updated_model = Solved(model.numbers, model.goal, solution)
  #(updated_model, effect.none())
}

fn reset_problem(_model: Model) -> #(Model, Effect(Msg)) {
  let updated_model = Setup(numbers_fields.new(), goal_field.new())
  #(updated_model, effect.none())
}

fn view(model: Model) -> Element(Msg) {
  element.fragment([
    header([], [h1([], [text("Letters and Numbers")])]),
    html.main([], [
      case model {
        Setup(numbers, goal) -> view_setup(numbers, goal)
        Solving(numbers, goal) -> view_solving(numbers, goal)
        Solved(numbers, goal, solution) -> view_solved(numbers, goal, solution)
      },
    ]),
    footer([], [text("Copyright © 2026; Nigel Eke. All rights reserved.")]),
  ])
}

fn view_setup(numbers: NumbersFields, goal: GoalField) -> Element(Msg) {
  section([class("main")], [
    numbers_fields_input.view(numbers, NumbersChanged),
    goal_field_input.view("goal", goal, PendingGoalChanged),
    button([on_click(SolutionRequested)], [text("Find Solution")]),
  ])
}

fn view_solving(numbers: NumbersFields, goal: GoalField) -> Element(Msg) {
  section([class("main")], [
    numbers_fields_read_only.view(numbers),
    goal_field_read_only.view(goal),
    text("Solving..."),
  ])
}

fn view_solved(
  numbers: NumbersFields,
  goal: GoalField,
  solution: SolutionField,
) -> Element(Msg) {
  section([class("main")], [
    numbers_fields_read_only.view(numbers),
    goal_field_read_only.view(goal),
    solution_field_read_only.view(solution),
    button([on_click(ResetRequested)], [text("Reset")]),
  ])
}
