package com.nigeleke.lettersandnumbers
package laminar

import com.raquo.laminar.api.L.{*, given}

def desktop(
             divModifiers: Mod[Div]
           )
: HtmlElement =

  val validatedNumbers: Var[ValidatedNumbers] = Var(None)
  val validatedGoal: Var[ValidatedGoal] = Var(None)
  val revealGoal = Var(false)

  div(
    goal(
      revealGoal.signal,
      validatedGoal.writer
    ),

    numbers(
      validatedNumbers.writer
    ),

    solution(
      validatedNumbers.signal,
      validatedGoal.signal,
      revealGoal.writer
    ),

    divModifiers
  )
