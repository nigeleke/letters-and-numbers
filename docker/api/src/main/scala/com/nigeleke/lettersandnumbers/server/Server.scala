package com.nigeleke.lettersandnumbers.server

import com.nigeleke.lettersandnumbers.api.*
import cats.effect.*
import com.typesafe.config.*
import org.http4s.*
import org.http4s.dsl.*
import org.http4s.dsl.io.*
import org.http4s.blaze.server.*
import org.http4s.headers.*
import org.http4s.implicits.*
import org.http4s.server.*
import org.http4s.server.middleware.*

object Server extends IOApp.Simple:

  given config: Config = ConfigFactory.load()

  override def run =
    val host = config.getString("lettersandnumbers.apiserver.host")
    val port = config.getInt("lettersandnumbers.apiserver.port")

    val services = CORS.policy.withAllowOriginAll(v1.Api.service).orNotFound

    var serverIo = BlazeServerBuilder[IO]
      .bindHttp(port, host)
      .withHttpApp(services)
      .resource
      .use(_ => IO.never)

    for _ <- serverIo
    yield ()
