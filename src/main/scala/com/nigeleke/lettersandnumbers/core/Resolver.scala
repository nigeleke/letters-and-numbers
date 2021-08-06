package com.nigeleke.lettersandnumbers.core

object Resolver:

  def findSolutions(
    operands: Seq[Int],
    goal: Int): Seq[String] =

    for
      permutation <- operands.permutationOfCombinations
      expression <- permutation.expressions if expression.value == goal
    yield expression.toString


extension (values: Seq[Int])

  def permutationOfCombinations: Seq[Seq[Int]] =
    (for
      size <- 1 to 6
      combination <- values.combinations(size)
      permutation <- combination.permutations
    yield permutation).toSeq

  def expressions: Seq[Expression] = 
    values match
      case Nil => Seq.empty
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