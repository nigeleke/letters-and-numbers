package com.nigeleke.lettersandnumbers
package laminar

import com.raquo.laminar.api.L.{*, given}

val desktopPanel =
  desktop(
    cls("w3-center")
  )

val headerPanel =
  header(
    "Letters and numbers",
    cls("w3-center w3-theme-d1 w3-xxlarge")
  )

val footerPanel =
  footer(
    "Copyright (C) 2021; Nigel Eke. All rights reserved.",
    cls("w3-bottom w3-center w3-theme-d3 w3-tiny")
  )

val rootElement =
  div(
    headerPanel,
    desktopPanel,
    footerPanel
  )
