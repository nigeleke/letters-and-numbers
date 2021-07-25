package com.nigeleke.lettersandnumbers

import com.raquo.laminar.api.L._

def ValidatedIntegerInput(
  n: Var[Option[Int]],
  validate: Option[Int] => Boolean,
  modifiers: Mod[HtmlElement]) : HtmlElement =

  val isValidSignal = n.signal.map(validate)

  span(
    input(
      controlled(
        value <-- n.signal.map((maybeInt: Option[Int]) => if maybeInt.isDefined then maybeInt.get.toString else ""),
        onInput.mapToValue.map(_.toIntOption) --> n
      ),
      textAlign.center,
      cls := "w3-round-large w3-theme-l3").amend(modifiers),
    WarningIcon(isValidSignal))
