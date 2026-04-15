import lustre/attribute.{class}
import lustre/element
import lustre/element/html.{div, text}

import domain/solution
import model/solution_field.{type SolutionField, Failed, Responded}

pub fn view(field: SolutionField) -> element.Element(msg) {
  let value = case field {
    Responded(s) -> solution.to_string(s)
    Failed(s) -> "Server error: " <> s
  }

  div([class("solution-field")], [text(value)])
}
