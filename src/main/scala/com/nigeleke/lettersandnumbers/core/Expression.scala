package com.nigeleke.lettersandnumbers.core

trait Expression(val value: Int)

case class Operand(override val value: Int) extends Expression(value) {
  override def toString(): String = s"$value"
}

case class Operation(override val value: Int,
                     operator: Operator,
                     left: Expression,
                     right: Expression) extends Expression(value) {
  override def toString(): String = s"($left $operator $right)"
}
