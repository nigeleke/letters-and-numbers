package com.nigeleke.lettersandnumbers

import com.raquo.laminar.api.L._

def ValidatedGoalInput(n: Var[Option[Int]]): HtmlElement =

  def validate(maybeInt: Option[Int]) =
    maybeInt.isEmpty ||
    maybeInt.map(i => 100 <= i && i <= 999).getOrElse(false)

  val styles = List(placeholder := "auto", cls := "w3-xxlarge", size := 3)

  ValidatedIntegerInput(n, validate, styles)
