package com.nigeleke.lettersandnumbers.core

import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class ReolverTestSpec extends AnyWordSpec with Matchers:

  "A Resolver" should {

    "return a None result for an impossible goal" in {
      val results = Resolver.findSolutions(Seq(1, 2, 3, 4, 5, 6), 999)
      results should be(empty)
    }

    "return valid expressions for a possible goal" in {
      val results = Resolver.findSolutions(Seq(1, 2, 3, 4, 5, 6), 720)
      results.size should be >(0)
    }

  }

  "The Resolvers listToExpression extensions" should {

    "not create any expressions from an empty list" in {
        val expressions = Seq.empty[Int].expressions
        expressions should be(empty)
    }

    "be a single operand when created from a single element list" in {
        val expressions = Seq(1).expressions
        expressions.size should be(1)
    }

    "be single operands and all valid operations when created from a two element list" in {
        val listResultMap = Map(
          Seq(1, 2) -> Seq(
            Operand(1),
            Operand(2),
            Operation(3, Operator.Plus, Operand(1), Operand(2))),
          Seq(4, 2) -> Seq(
            Operand(4),
            Operand(2),
            Operation(6, Operator.Plus, Operand(4), Operand(2)),
            Operation(2, Operator.Minus, Operand(4), Operand(2)),
            Operation(8, Operator.Times, Operand(4), Operand(2)),
            Operation(2, Operator.Divides, Operand(4), Operand(2)))
        )

        listResultMap.foreach { (list, expectedExpressions) =>
          val expressions = list.expressions
          expressions should contain theSameElementsAs(expectedExpressions)
        }
    }

  }