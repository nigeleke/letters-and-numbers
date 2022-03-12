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

  def bestExpressions(operands: Seq[Int], goal: Int): Seq[Expression] =
    def operationCount(e: Expression): Int =
      e match {
        case Operand(_)            => 0
        case Operation(_, _, l, r) => operationCount(l) + operationCount(r)
      }
    (for
      permutation <- operands.permutationOfCombinations
      expression <- permutation.expressions if expression.value == goal
    yield expression).sortBy(operationCount)

  def findSolutions(operands: Seq[Int], goal: Int): Seq[String] =
    bestExpressions(operands, goal).map(_.toString)

extension (values: Seq[Int])

  def permutationOfCombinations: Seq[Seq[Int]] =
    (for
      size <- 1 to 6
      combination <- values.combinations(size)
      permutation <- combination.permutations
    yield permutation).toSeq

  def expressions: Seq[Expression] =
    values match
      case Nil      => Seq.empty
      case x :: Nil => Seq(Operand(x))
      case _ =>
        val operandExpressions = values.map(Operand(_))
        val operationExpressions =
          for
            splitIndex <- 1 until values.size
            (left, right) = values.splitAt(splitIndex)
            leftExpression <- left.expressions
            rightExpression <- right.expressions
            operator <- Operator.values
            operationValue <- operator(leftExpression.value, rightExpression.value)
          yield Operation(operationValue, operator, leftExpression, rightExpression)
        operandExpressions ++ operationExpressions
