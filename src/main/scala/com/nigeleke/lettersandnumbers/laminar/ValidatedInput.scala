package com.nigeleke.lettersandnumbers
package laminar

import com.raquo.laminar.api.L.*

def validatedInput[T](
                       validate: String => Validated[T],
                       observer: Observer[Validated[T]],
                       inputModifiers: Mod[Input]*
                     )
: HtmlElement =

  val stringVar = Var("")

  val validated = stringVar.signal.map(validate)
  val isValid = validated.map(_.isDefined)
  val notifications = validated.changes

  def initialiseObservers(context: MountContext[_]) =
    notifications.addObserver(observer)(context.owner)

  span(
    onMountCallback(initialiseObservers),
    input(
      controlled(
        value <-- stringVar,
        onInput.mapToValue --> stringVar
      ),
      cls("w3-round-large w3-theme-l3"),
      inputModifiers
    ),
    warningIcon(isValid.map(!_))
  )
