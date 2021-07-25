// package com.nigeleke.lettersandnumbers

// import munit.*
// import org.scalacheck.*
// import org.scalacheck.Prop.*

// class ExpressionTestSpec extends ScalaCheckSuite:

//   val nOperands = Gen.choose(1, 6)

//   property("The number of Expressions of n Operands in a single Leaf Tree is n") {
//     forAll(nOperands) { n =>
//       val operands = List.fill[Int](n)((Math.random() * 10).toInt)
//       val expressions = Expression.fromTree(Leaf, operands)
//       assertEquals(expressions.size, n)
//     }
//   }

//   val nOperands2 = Gen.choose(2, 6)

//   property("The number of Expressions of n Operands in an 2 Leaf Tree is 4(operations) * nP2") {
//     def factorial(n: Int): Int = if n == 0 then 1 else n * factorial(n-1)
//     def nPr(n: Int, r: Int) = factorial(n) / factorial(n-r)
//     forAll(nOperands2) { n =>
//       val operands = List.fill[Int](n)((Math.random() * 10).toInt)
//       val expressions = Expression.fromTree(Branch(Leaf, Leaf), operands)
//       assertEquals(expressions.size, 4 * nPr(n, 2))
//     }
//   }

//   val value = Gen.oneOf(Range(1, 10) ++ Range(1, 10) ++ Seq(25, 50, 75, 100))
//   val operandGen = for (v <- value) yield Operand(v, 0)
//   def operationGen(depthLimit: Int): Gen[Expression] =
//     for
//       operation <- Gen.oneOf(Plus(_, _), Minus(_, _), Times(_, _), Divide(_, _))
//       left <- genExpression(depthLimit - 1)
//       right <- genExpression(depthLimit - 1)
//     yield operation(left, right)

//   def genExpression(depthLimit: Int) = 
//     if (depthLimit == 0)
//     then operandGen
//     else operationGen(depthLimit)

//   def commute(e: Expression): Expression = e match
//     case o: Operand => o
//     case Plus(left, right) => Plus(commute(right), commute(left))
//     case m: Minus => m
//     case Times(left, right) => Times(commute(right), commute(left))
//     case d: Divide => d

//   property("Expressions are deemed equal when commutative Operations have Operands swapped") {
//     forAll(genExpression(4)) { (expression: Expression) =>
//       assert(commute(expression).equivalent(expression), s"Commuted expression not equivalent $expression")
//     }
//   }

//   property("A Set of equivalent expressions will only include one expression") {
//     forAll(genExpression(4)) { (expression: Expression) => 
//       val expressions = Set(expression, commute(expression))
//       assertEquals(expressions.size, 1)
//     }
//   }
