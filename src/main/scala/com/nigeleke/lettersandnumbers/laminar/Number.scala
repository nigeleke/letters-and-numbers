package com.nigeleke.lettersandnumbers
package laminar

import com.raquo.laminar.api.L.{*, given}

def number(
            observer: Observer[ValidatedNumber]
          )
: HtmlElement =

  val validate: String => ValidatedNumber = { s =>
    val validNumbers = smallNumbers ++ largeNumbers
    s.toIntOption.filter(validNumbers.contains)
  }

  validatedInput(
    validate,
    observer,
    title("Number"),
    textAlign.center,
    size(2),
    cls("w3-xxlarge")
  )
