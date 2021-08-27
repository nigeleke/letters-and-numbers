package com.nigeleke.lettersandnumbers.core

enum Operator(operation: (Int, Int) => Option[Int]):

  def apply(a: Int, b: Int): Option[Int] = operation(a, b)

  override def toString: String = this match
    case Plus    => "+"
    case Minus   => "-"
    case Times   => "*"
    case Divides => "/"

  case Plus    extends Operator( (a, b) => Some(a + b) )
  case Minus   extends Operator( (a, b) => if a > b then Some(a - b) else None)
  case Times   extends Operator( (a, b) => if a != 1 && b != 1 then Some(a * b) else None)
  case Divides extends Operator( (a, b) => if b != 1 && a % b == 0 then Some(a / b) else None)
