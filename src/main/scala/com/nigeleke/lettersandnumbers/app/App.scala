package com.nigeleke.lettersandnumbers
package app

import laminar.*

import org.scalajs.dom
import org.scalajs.dom.document
import com.raquo.laminar.api.L.*

object App:

  def main(unused: Array[String]): Unit =
    def init(unused: dom.Event): Unit = render(document.body, rootElement)
    document.addEventListener("DOMContentLoaded", init _)
