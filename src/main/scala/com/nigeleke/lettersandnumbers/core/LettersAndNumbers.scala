package com.nigeleke.lettersandnumbers

object LettersAndNumbers:
  def findSolutions(operands: Seq[Int], goal: Int): Seq[String] =
    for
      n <- Range(1, 6)
      tree <- Tree.structures(n)
      expression <- Expression.fromTree(tree, operands).toSet
      result <- expression.value if result == goal
    yield expression.toString

  def findSolution(operands: Seq[Int], goal: Int): Option[String] =
    findSolutions(operands, goal).headOption

// @main def main(
//   v1: Int,
//   v2: Int,
//   v3: Int,
//   v4: Int,
//   v5: Int,
//   v6: Int,
//   goal: Int): Unit =
//   val maybeResult = LettersAndNumbers.findSolution(Seq(v1, v2, v3, v4, v5, v6), goal)
//   println(maybeResult.getOrElse("No solution"))
