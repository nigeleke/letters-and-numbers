package com.nigeleke.lettersandnumbers.core

import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class DefectsTestSpec extends AnyWordSpec with Matchers:

  "Failed findSolutions" should {

    "actually have a solution" in {
      val failedSolutions = Map(
        Seq(75, 3, 6, 5, 5, 1) -> 559,
        Seq(5, 8, 8, 2, 100, 50) -> 543
      )

      failedSolutions.foreach { (numbers, goal) =>
        val results = Resolver.findSolutions(numbers, goal)
        results.size should be > (0)
      }
    }
  }
