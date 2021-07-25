package com.nigeleke.lettersandnumbers

sealed trait Expression:
  override def equals(that: Any): Boolean =
    if that.isInstanceOf[Expression]
    then this.equivalent(that.asInstanceOf[Expression])
    else false

  def equivalent(that: Expression): Boolean

case class Operand(value: Int, id: Int) extends Expression:
  override def toString = s"$value"
  def equivalent(that: Expression): Boolean = that match
    case Operand(value, id) => true
    case _ => false

case class Plus(a: Expression, b: Expression) extends Expression:
  override def toString = s"($a + $b)"
  def equivalent(that: Expression): Boolean = that match
    case Plus(thatLeft, thatRight) =>
      (a.equivalent(thatLeft) && b.equivalent(thatRight)) ||
      (a.equivalent(thatRight) && b.equivalent(thatLeft))
    case _ => false

case class Minus(a: Expression, b: Expression) extends Expression:
  override def toString = s"($a - $b)"
  def equivalent(that: Expression): Boolean = that match
    case Minus(thatLeft, thatRight) =>
      (a.equivalent(thatLeft) && b.equivalent(thatRight))
    case _ => false

case class Times(a: Expression, b: Expression) extends Expression:
  override def toString = s"($a * $b)"
  def equivalent(that: Expression): Boolean = that match
    case Times(thatLeft, thatRight) =>
      (a.equivalent(thatLeft) && b.equivalent(thatRight)) ||
      (a.equivalent(thatRight) && b.equivalent(thatLeft))
    case _ => false

case class Divide(a: Expression, b: Expression) extends Expression:
  override def toString = s"($a / $b)"
  def equivalent(that: Expression): Boolean = that match
    case Divide(thatLeft, thatRight) =>
      (a.equivalent(thatLeft) && b.equivalent(thatRight))
    case _ => false

type Operation = (Expression, Expression) => Expression

object Expression:
  def fromTree(
    tree: Tree,
    values: Seq[Int]): Seq[Expression] =

    val operands = values.zipWithIndex.map(Operand(_, _))
    
    def nextOperand(iterator: Iterator[Operand])(): Seq[Expression] = Seq(iterator.next)

    def operations(lefts: Seq[Expression], rights: Seq[Expression]) =
      for
        left <- lefts
        right <- rights
        operation <- Seq(Plus(_, _), Minus(_, _), Times(_, _), Divide(_, _))
      yield operation(left, right)

    val size = tree.leafSize

    (for
      combination <- operands.combinations(size)
      permutation <- combination.permutations
      iterator = permutation.iterator
      expression <- tree.map(nextOperand(iterator), operations)
    yield expression).toSeq

extension (expression: Expression)
  def value: Option[Int] = expression match
    case Operand(v, _) => Some(v)

    case Plus(e1, e2) => for {
      v1 <- e1.value
      v2 <- e2.value
    } yield v1 + v2

    case Minus(e1, e2) =>
      def positiveResult(v1: Int, v2: Int) = v1 > v2
      for {
        v1 <- e1.value
        v2 <- e2.value if positiveResult(v1, v2)
      } yield v1 - v2

    case Times(e1, e2) =>
      def trivial(v: Int) = v == 1
      for {
        v1 <- e1.value if !trivial(v1)
        v2 <- e2.value if !trivial(v2)
      } yield v1 * v2

    case Divide(e1, e2) =>
      def trivial(v: Int) = v == 1
      def error(v: Int) = v == 0
      def exact(v1: Int, v2: Int) = v1 % v2 == 0
      for {
        v1 <- e1.value
        v2 <- e2.value if !error(v2) && !trivial(v2) && exact(v1, v2)
      } yield v1 / v2