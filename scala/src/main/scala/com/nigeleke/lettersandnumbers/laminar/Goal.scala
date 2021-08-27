package com.nigeleke.lettersandnumbers
package laminar

import com.nigeleke.lettersandnumbers.core.Resolver
import com.raquo.laminar.api.L.*

import scala.util.Random

def goal(
          revealed: Observable[Boolean],
          observer: Observer[ValidatedGoal]
        )
: HtmlElement =

  type ValidatedGoalString = Validated[String]
  val inputtedGoal: Var[ValidatedGoalString] = Var(Some(""))

  def randomGoal = Random.nextInt(900) + 100

  val target =
    inputtedGoal
      .signal
      .map {
        case Some(goalString) => Some(if goalString.isEmpty then randomGoal else goalString.toInt)
        case None => None
      }

  val validate : String => ValidatedGoalString = { s =>
    val auto = s.isEmpty
    val validInt = s.toIntOption.forall((100 to 999).contains)
    if auto then Some("")
    else if validInt then Some(s)
    else None
  }

  val onReveal =
    Signal
      .combine(target, revealed.toWeakSignal)

  def initialiseObservers(context: MountContext[_]) =
    target.addObserver(observer)(context.owner)

  div(
    onMountCallback(initialiseObservers),
    validatedInput(
      validate,
      inputtedGoal.writer,
      placeholder("auto"),
      title("Target"),
      textAlign.center,
      size(3),
      cls("w3-xxxlarge")
    ),
    cls("w3-center w3-section")
  )
