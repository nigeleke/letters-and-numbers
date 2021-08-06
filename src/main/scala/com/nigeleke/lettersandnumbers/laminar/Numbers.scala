package com.nigeleke.lettersandnumbers
package laminar

import com.raquo.laminar.api.L.{*, given}

def numbers(
             observer: Observer[ValidatedNumbers]
           )
: HtmlElement =

  val numbers = (1 to 6).map(_ => Var(Option.empty[Number]))
  val numbersSignals = Signal.sequence(numbers.map(_.signal))

  def validate(validatedNumbers: Seq[ValidatedNumber]): ValidatedNumbers =
    val numbers = validatedNumbers.filter(_.isDefined).map(_.get)
    val availableNumbers = smallNumbers ++ smallNumbers ++ largeNumbers
    val unusedNumbers = availableNumbers diff numbers
    if (unusedNumbers.size == 18) Some(numbers) else None

  val notifications = numbersSignals.changes.map(validate)

  def initialiseObservers(context: MountContext[_]) =
    notifications.addObserver(observer)(context.owner)

  div(
    onMountCallback(initialiseObservers),
    numbers.map(v => number(v.writer)),
    cls("w3-center w3-section w3-auto w3-row w3-row-padding")
  )
