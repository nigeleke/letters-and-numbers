package com.nigeleke.lettersandnumbers
package laminar

import com.raquo.laminar.api.L.{*, given}

def warningIcon(
                 required: Observable[Boolean]
               )
: Mod[HtmlElement] =

  i(
    position("absolute"),
    marginLeft("-18px"),
    marginTop("6px"),
    display <-- required.map(if _ then null else "none"),
    cls("fa fa-warning w3-small w3-text-theme")
  )
