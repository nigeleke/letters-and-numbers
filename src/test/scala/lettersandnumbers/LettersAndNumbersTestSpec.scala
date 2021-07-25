// package com.nigeleke.lettersandnumbers

// import munit.*

// class LettersAndNumbersTestSpec extends FunSuite {

//   test("An impossible Goal will return a None result") {
//     val results = LettersAndNumbers.findSolutions(Seq(1, 2, 3, 4, 5, 6), 999)
//     assert(results.isEmpty, s"Expected no results, have: ${results.size}")
//   }

//   test("A possible Goal will return valid Expressions") {
//     val results = LettersAndNumbers.findSolutions(Seq(1, 2, 3, 4, 5, 6), 720)
//     assert(results.size > 0, s"Expected single results, have: ${results.size}")
//   }

// }
