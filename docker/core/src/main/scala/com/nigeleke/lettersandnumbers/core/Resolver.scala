package com.nigeleke.lettersandnumbers.core

object Resolver:

  def bestSolution(operands: Seq[Int], goal: Int): Either[String, Option[String]] =
    def isValidOperands(set: Seq[Int], inCountLimit: Int => Boolean) =
      operands
        .filter(set.contains)
        .groupMapReduce(identity)(_ => 1)(_ + _)
        .values
        .forall(inCountLimit)
    val isValidLowOperands = isValidOperands(1 to 10, _ <= 2)
    val isValidHighOperands = isValidOperands(Seq(25, 50, 75, 100), _ <= 1)
    val isValidNumbers = isValidLowOperands && isValidHighOperands
    val isValidGoal = (100 to 999).contains(goal)
    if (!isValidNumbers) Left("Invalid numbers")
    else if (!isValidGoal) Left("Invalid goal")
    else Right(findSolutions(operands, goal).headOption)

  def findSolutions(operands: Seq[Int], goal: Int): Seq[String] =
    bestExpressions(operands, goal).map(_.toString)

  def bestExpressions(operands: Seq[Int], goal: Int): Seq[Expression] =
    def operationCount(e: Expression): Int =
      e match {
        case Operand(_)            => 0
        case Operation(_, _, l, r) => operationCount(l) + operationCount(r)
      }
    val expressions =
      (for
        permutation <- operands.permutations
        expression <- permutation.expressions if expression.value == goal
      yield expression).toSeq
    expressions.sortBy(operationCount)

extension (values: Seq[Int])
  def expressions: Seq[Expression] =
    def operations(left: Expression, right: Expression): Seq[Expression] =
      for
        operator <- Operator.values
        value <- operator(left.value, right.value)
      yield Operation(value, operator, left, right)

    values match
      case Nil      => Seq.empty
      case x :: Nil => Seq(Operand(x))
      case _ =>
        (for
          i <- 1 to values.size - 1
          (lops, rops) = values.splitAt(i)
          (les, res) = (lops.expressions, rops.expressions)
          appliedExpressions = (for
            le <- les
            re <- res
          yield operations(le, re)).flatten
        yield les ++ res ++ appliedExpressions).flatten
