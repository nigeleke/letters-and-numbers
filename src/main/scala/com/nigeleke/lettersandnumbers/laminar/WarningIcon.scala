package com.nigeleke.lettersandnumbers

import com.raquo.laminar.api.L._

def WarningIcon(
  validSignal: Signal[Boolean]) : HtmlElement =

  val displayModifier = validSignal.map(if _ then "none" else "inline")
  
  i(
    position("absolute"),
    marginLeft("-18px"),
    marginTop("6px"),
    display <-- displayModifier,
    cls := "fa fa-warning w3-small w3-text-theme")
