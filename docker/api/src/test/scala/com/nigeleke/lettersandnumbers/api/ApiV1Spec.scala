package com.nigeleke.lettersandnumbers.api

import cats.data.*
import cats.effect.*
import cats.effect.testing.scalatest.*
import org.http4s.*
import org.http4s.dsl.io.*
import org.http4s.headers.*
import org.http4s.implicits.*
import org.scalatest.*
import wordspec.*
import matchers.should.*
import org.scalacheck.*
import org.scalatestplus.scalacheck.*

import scala.concurrent.*

class ApiV1Spec extends AsyncWordSpec with AsyncIOSpec with Matchers:

  def makeSolveUri(numbers: Seq[Int], maybeGoal: Option[Int]) =
    val nis = numbers.zipWithIndex
    val params =
      nis.map(ni => s"n${ni._2 + 1}" -> s"${ni._1}") ++
        maybeGoal.map(g => s"goal" -> s"$g")
    uri"/api/v1/solve".withQueryParams(params.toMap)

  "The API V1" should {

    val serviceUnderTest = v1.Api.service.orNotFound

    "return Ok Some solution when solvable request invoked" in {
      val ns = Seq(1, 2, 3, 4, 5, 6)
      val goal = 720
      val request = Request[IO](Method.GET, makeSolveUri(ns, Some(goal)))
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.Ok)
      }
      serviceUnderTest.run(request).flatMap(_.as[String]).asserting { body =>
        body should be("(2 * (3 * (4 * (5 * 6))))")
      }
    }

    "return Ok None when unsolvable request invoked" in {
      val ns = Seq(1, 2, 3, 4, 5, 6)
      val goal = 999
      val request = Request[IO](Method.GET, makeSolveUri(ns, Some(goal)))
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.Ok)
      }
      serviceUnderTest.run(request).flatMap(_.as[String]).asserting { body =>
        body should be("No Solution")
      }
    }

    "return BadRequest when invalid numbers provided in request" in {
      val ns = Seq(1, 1, 1, 25, 25, 6)
      val goal = 999
      val request = Request[IO](Method.GET, makeSolveUri(ns, Some(goal)))
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.BadRequest)
      }
      serviceUnderTest.run(request).flatMap(_.as[String]).asserting { body =>
        body should be("Invalid numbers")
      }
    }

    "return BadRequest when invalid goal provided in request" in {
      val ns = Seq(1, 2, 3, 4, 5, 6)
      val goal = 9999
      val request = Request[IO](Method.GET, makeSolveUri(ns, Some(goal)))
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.BadRequest)
      }
      serviceUnderTest.run(request).flatMap(_.as[String]).asserting { body =>
        body should be("Invalid goal")
      }
    }

    "return NotFound when invalid url provided" in {
      val request = Request[IO](Method.GET, uri"/none")
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.NotFound)
      }
    }

    "return NotFound when not all query parameters provided 1" in {
      val ns = Seq(2, 3, 4, 5, 6)
      val goal = 999
      val request = Request[IO](Method.GET, makeSolveUri(ns, Some(goal)))
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.NotFound)
      }
    }

    "return NotFound when not all query parameters provided 2" in {
      val ns = Seq(1, 2, 3, 4, 5, 6)
      val request = Request[IO](Method.GET, makeSolveUri(ns, None))
      serviceUnderTest.run(request).asserting { response =>
        response.status should be(Status.NotFound)
      }
    }

  }
