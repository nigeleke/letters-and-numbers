package com.nigeleke.lettersandnumbers
package laminar

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

import com.raquo.laminar.api.L.{*, given}

class WarningIconTestSpec  extends AnyWordSpec with Matchers:

  // TODO: Test analysis required;
  //       Expected versus actual comparison not being made as expected...
  "A Warning Icon" should {

    "be visible if warning required" ignore {
      val warningSignal = Signal.fromValue(true)
      val icon = warningIcon(warningSignal)
      icon.toString should be(
        i(
          position("absolute"),
          marginLeft("-18px"),
          marginTop("6px"),
          cls("fa fa-warning w3-small w3-text-theme")
        ).toString
      )
    }

    "be hidden if warning not required" ignore {
      val warningSignal = Signal.fromValue(false)
      val icon = warningIcon(warningSignal)
      icon.toString should be(
        i(
          position("absolute"),
          marginLeft("-18px"),
          marginTop("6px"),
          // display("none"),
          cls("fa fa-warning w3-small w3-text-theme")
        ).toString
      )
    }

  }