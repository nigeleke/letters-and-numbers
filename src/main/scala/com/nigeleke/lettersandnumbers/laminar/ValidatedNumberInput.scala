package com.nigeleke.lettersandnumbers

import com.raquo.laminar.api.L._

def ValidatedNumberInput(n: Var[Option[Int]]): HtmlElement =

  def validate(maybeInt: Option[Int]) =
    val validNumbers = (1 to 10) ++ Seq(25, 50, 75, 100)
    validNumbers.contains(maybeInt.getOrElse(0))

  val styles = List(cls := "w3-xlarge", size := 2)

  ValidatedIntegerInput(n, validate, styles)
