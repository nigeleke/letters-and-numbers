package com.nigeleke.lettersandnumbers

import scala.util.Random

package object laminar:

  type Validated[T] = Option[T]

  type Number = Int
  type ValidatedNumber = Validated[Number]

  type Numbers = Seq[Number]
  type ValidatedNumbers = Validated[Numbers]

  type Goal = Int
  type ValidatedGoal = Validated[Goal]

  val smallNumbers = 1 to 10

  val largeNumbers = Seq(25, 50, 75, 100)
