ThisBuild / version := "0.1.0"
ThisBuild / parallelExecution := false
ThisBuild / scalaVersion := "3.0.1"

val scalatestVersion = "3.2.9"

lazy val root = (project in file("."))
  .enablePlugins(ScalaJSPlugin)
  .settings(
    name := "letters-and-numbers",
    libraryDependencies ++= Seq(
      "org.scala-js" % "scalajs-dom_sjs1_2.13" % "1.1.0",
      "com.raquo" %%% "laminar" % "0.13.1",
       "org.scalactic" %%% "scalactic" % scalatestVersion,
       "org.scalatest" %%% "scalatest" % scalatestVersion % Test),

    scalaJSUseMainModuleInitializer := true,

    jsEnv := new org.scalajs.jsenv.jsdomnodejs.JSDOMNodeJSEnv()
  )
