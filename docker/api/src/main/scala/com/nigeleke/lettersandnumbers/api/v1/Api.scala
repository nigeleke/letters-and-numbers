package com.nigeleke.lettersandnumbers.api.v1

import com.nigeleke.lettersandnumbers.core.*

import cats.effect.*
import org.http4s.*
import org.http4s.dsl.*
import org.http4s.dsl.io.*

object Api:

  object N1Param extends OptionalQueryParamDecoderMatcher[Int]("n1")
  object N2Param extends OptionalQueryParamDecoderMatcher[Int]("n2")
  object N3Param extends OptionalQueryParamDecoderMatcher[Int]("n3")
  object N4Param extends OptionalQueryParamDecoderMatcher[Int]("n4")
  object N5Param extends OptionalQueryParamDecoderMatcher[Int]("n5")
  object N6Param extends OptionalQueryParamDecoderMatcher[Int]("n6")
  object GoalParam extends OptionalQueryParamDecoderMatcher[Int]("goal")

  val service = HttpRoutes
    .of[IO] {
      case GET -> Root / "api" / "v1" / "solve" :?
          N1Param(optN1) :? N2Param(optN2) :? N3Param(optN3) :?
          N4Param(optN4) :? N5Param(optN5) :? N6Param(optN6) :?
          GoalParam(optGoal) =>
        val maybeResultOrError =
          for
            n1 <- optN1
            n2 <- optN2
            n3 <- optN3
            n4 <- optN4
            n5 <- optN5
            n6 <- optN6
            goal <- optGoal
          yield Resolver.bestSolution(Seq(n1, n2, n3, n4, n5, n6), goal)
        maybeResultOrError match {
          case Some(Right(maybeResult)) => Ok(maybeResult.getOrElse("No Solution"))
          case Some(Left(error))        => BadRequest(error)
          case None                     => NotFound()
        }
    }
