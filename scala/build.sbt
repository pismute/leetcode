lazy val root = project
  .in(file("."))
  .settings(
    name := "leetcode-scala",
    version := "0.1.0-SNAPSHOT",
    scalaVersion := "2.13.10",
    semanticdbEnabled := true // for metal
  )
