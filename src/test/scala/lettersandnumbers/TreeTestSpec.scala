// package com.nigeleke.lettersandnumbers

// import munit.*
// import org.scalacheck.*
// import org.scalacheck.Gen.*
// import org.scalacheck.Prop.*

// class TreeTestSpec extends ScalaCheckSuite:

//   import Tree.*

//   val genLeavesCount = Gen.choose(1, 6)

//   property("The number of tree structures for n leaves is n=1 => 1; n>1 => 2^(n-2)") {
//     forAll(genLeavesCount) { n =>
//       val trees = Tree.structures(n)
//       assertEquals(trees.size, if n == 1 then 1 else 1 << (n-2))
//     }
//   }

//   val genTreeWithLeavesCount = for {
//     n <- genLeavesCount
//     t <- Gen.oneOf(Tree.structures(n))
//   } yield (t, n)

//   property("map will call f 2n-1 times for an n leaf, n-1 branch tree") {
//     forAll(genTreeWithLeavesCount) { (t, n) => 
//       val leafCount = Iterator.from(0)
//       val branchCount = Iterator.from(0)
//       t.map( leafCount.next, (_, _) => branchCount.next)
//       assertEquals(leafCount.next, n)
//       assertEquals(branchCount.next, n-1)
//     }
//   }

//   property("leafSize will return n for a n leaf tree") {
//     forAll(genTreeWithLeavesCount) { (t, n) => 
//       assertEquals(t.leafSize, n)
//     }
//   }
