ThisBuild / version := "0.1.0"
ThisBuild / parallelExecution := false
ThisBuild / scalaVersion := "3.0.1"

val munitVersion = "0.7.26"

lazy val root = project
  .in(file("."))
  .enablePlugins(ScalaJSPlugin)
  .settings(
    name := "letters-and-numbers",
    scalaJSUseMainModuleInitializer := true,

    libraryDependencies ++= Seq(
      "org.scala-js" % "scalajs-dom_sjs1_2.13" % "1.1.0",
      "com.raquo" %%% "laminar" % "0.13.1"),
      // "org.scalameta" %% "munit" % munitVersion % Test,
      // "org.scalameta" %% "munit-scalacheck" % munitVersion % Test),

    jsEnv := new org.scalajs.jsenv.jsdomnodejs.JSDOMNodeJSEnv()
  )
 