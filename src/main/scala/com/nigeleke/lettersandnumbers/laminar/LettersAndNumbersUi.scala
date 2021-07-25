package com.nigeleke.lettersandnumbers

import org.scalajs.dom
import org.scalajs.dom.document
import com.raquo.laminar.api.L.*

import scala.util.*
import scala.concurrent.ExecutionContext.Implicits.global

val numbers = Seq.fill(6)(Var(Option.empty[Int]))
val goal = Var(Option.empty[Int])
val solution = Var(Option.empty[String])

val solutionPanel =
  val styles = cls := "w3-center w3-section w3-xlarge" 
  div(
    child <-- solution.signal.map(p(_)),
    styles)

val startPanel =
  def correctlySelectedNumbers(maybeNumbers: Seq[Option[Int]]): Boolean =
    val numbers = maybeNumbers.flatten
    val availableNumbers = (1 to 10) ++ (1 to 10) ++ Seq(25, 50, 75 ,100)
    val unusedNumbers = availableNumbers diff numbers
    unusedNumbers.size == 18

  val validNumbersSignal =
    Signal.sequence(numbers.map(_.signal)) compose (_.map(correctlySelectedNumbers))

  val validGoalSignal =
    goal.signal.map(maybeInt => maybeInt.isEmpty || maybeInt.map(i => 100 <= i && i <= 999).getOrElse(false))

  val valid : Signal[Boolean] = Signal.sequence(Seq(validGoalSignal, validNumbersSignal)).map(_.foldLeft(true)(_ && _))

  val panelStyles = cls := "w3-center"
  val buttonStyles = cls := "fa fa-play w3-padding-small w3-xlarge w3-round-large"

  div(
    button(
      disabled <-- valid.map(!_),
      color <-- validNumbersSignal.map(if _ then "none" else "grey"),
      onClick.map { _ =>
        val realNumbers = numbers.map(_.now().get)
        val maybeRealGoal = goal.now()
        val realGoal = maybeRealGoal.getOrElse(Random.nextInt(900) + 100)
        if maybeRealGoal.isEmpty then goal.set(Some(realGoal))
        Some(LettersAndNumbers.findSolution(realNumbers, realGoal).getOrElse("Not possible"))
      } --> solution.writer,
      buttonStyles
    ),
    panelStyles
  )

val numbersPanel =
  val styles = cls := "w3-center w3-section w3-auto w3-row w3-row-padding"
  div(
    numbers.map(ValidatedNumberInput),
    styles
  )

val goalPanel =
  val styles = cls := "w3-center w3-section" 
  div(ValidatedGoalInput(goal), styles)

val desktopPanel =
  div(
    goalPanel,
    numbersPanel,
    startPanel,
    solutionPanel)

val headerPanel =
  val styles = cls := "w3-center w3-theme-d1 w3-xxlarge"
  header("Letters and Numbers", styles)

val footerPanel =
  val styles = cls := "w3-bottom w3-center w3-theme-d3 w3-tiny"
  footer("Copyright (C) 2021; Nigel Eke. All rights reserved.", styles)

val rootElement = div(
  headerPanel,
  desktopPanel,
  footerPanel)

object LettersAndNumbersUi:
  def main(unused: Array[String]): Unit =
    def init(unused: dom.Event): Unit = render(document.body, rootElement)
    document.addEventListener("DOMContentLoaded", init _)
