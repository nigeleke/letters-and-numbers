package com.nigeleke.lettersandnumbers
package laminar

import com.nigeleke.lettersandnumbers.core.Resolver
import com.raquo.airstream.core.Observable
import com.raquo.laminar.api.L.*
import org.scalajs.dom.*

import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.Future
import scala.concurrent.Promise
import scala.util.Random

def solution(
              observableNumbers: Observable[ValidatedNumbers],
              observableGoal: Observable[ValidatedGoal],
              revealGoal: Observer[Boolean]
            )
: HtmlElement =

  val maybeNumbers = observableNumbers.toWeakSignal.map(_.get)
  val actualNumbers = maybeNumbers.map {
    case None     => Seq.empty // Don't care - not valid - won't get used
    case Some(xs) => xs
  }

  val maybeGoal = observableGoal.toWeakSignal.map(_.get)
  val actualGoal = maybeGoal.map {
    case None => -1 // Don't care - not valid - won't get used
    case Some(g) => g
  }

  val validInput =
    Signal.combine(
      maybeNumbers.map(_.isDefined),
      maybeGoal.map(_.isDefined)
    ).map(_ && _)

  val solving = Var(false)
  val maybeResult = Var(Option.empty[String])
  val result = maybeResult.signal.changes.filter(_.isDefined).map(_.get)

  val findSolution =
    Observer[Event](onNext = ev => {
      Var(true) --> revealGoal
      solving.set(true)

      import com.raquo.airstream.ownership.*
      implicit val owner = OneTimeOwner(() => {})
      val promise = Promise[String]()
      val eventuallyResult = Signal.fromFuture(promise.future)
      eventuallyResult.addObserver(maybeResult.writer)

      promise.success {
        import com.raquo.airstream.ownership.*
        implicit val owner = OneTimeOwner(() => { })
        val result =
          Signal.combine(actualNumbers, actualGoal)
            .map(Resolver.findSolutions)
            .map(_.headOption.getOrElse("No solution"))
        solving.set(false)
        result.observe.now()
      }
    })

  val disabledFindSolution =
    Signal.combine(
      validInput.map(!_),
      solving
    ).map(_ || _)

  div(
    cls("w3-small-text"),
    button(
      title("Find solution"),
      onClick --> findSolution,
      disabled <-- disabledFindSolution,
      cls("fa fa-play w3-theme-text")
    ),
    div(
      div("Numbers: ", child.text <-- maybeNumbers.map(_.toString)),
      " ",
      div("Goal: ", child.text <-- maybeGoal.map(_.toString)),
      cls("w3-opacity w3-large")
    ),
    p(child.text <-- result),
    cls("w3-center w3-section w3-xlarge"))
