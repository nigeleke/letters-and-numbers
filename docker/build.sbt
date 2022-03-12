val scala3Version = "3.1.1"

ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / scalaVersion := scala3Version

val catsEffectTestingVersion = "1.4.0"
val http4sVersion = "1.0.0-M31"
val laminarVersion = "0.14.2"
val scalacheckVersion = "3.2.11.0"
val scalaJsDomVersion = "2.1.0"
val scalatestVersion = "3.2.11"

lazy val root = project
  .in(file("."))
  .settings(name := "letter-and-numbers")
  .aggregate(api, core, ui)

lazy val api = project
  .settings(
    name := "api",
    libraryDependencies ++= Seq(
      "org.http4s" %% "http4s-dsl" % http4sVersion,
      "org.http4s" %% "http4s-server" % http4sVersion,
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test",
      "org.scalatestplus" %% "scalacheck-1-15" % scalacheckVersion % "test",
      "org.typelevel" %% "cats-effect-testing-core" % catsEffectTestingVersion % "test",
      "org.typelevel" %% "cats-effect-testing-scalatest" % catsEffectTestingVersion % "test"
    )
  )
  .dependsOn(core)

lazy val core = project
  .settings(
    name := "core",
    libraryDependencies ++= Seq(
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
    )
  )

lazy val ui = project
  .settings(
    name := "ui"
  )
