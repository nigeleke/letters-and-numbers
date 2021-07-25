package com.nigeleke.lettersandnumbers

sealed trait Tree
case object Leaf extends Tree:
  override def toString() = "Leaf"
case class Branch(left: Tree, right: Tree) extends Tree:
  override def toString() = s"($left, $right)"

extension(t: Tree)
  def map[A](f: () => A, g: (A, A) => A): A = t match
    case Leaf => f()
    case Branch(left, right) => g(left.map(f, g), right.map(f, g))
  
  def leafSize: Int = t match
    case Leaf => 1
    case Branch(left, right) => left.leafSize + right.leafSize

object Tree:
  def structures(nLeaves: Int): Seq[Tree] =
    def structuresSet(n: Int): Set[Tree] =
      n match
        case 0 => Set.empty
        case 1 => Set(Leaf)
        case _ => for {
          s <- structuresSet(n - 1)
          l = Leaf
          b <- Set(Branch(s, l), Branch(l, s))
        } yield b
    structuresSet(nLeaves).toSeq
