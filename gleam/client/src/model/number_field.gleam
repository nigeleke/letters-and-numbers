import domain/number.{type Number, type ValidationError}

pub type NumberField {
  NumberField(raw: String, validated: Result(Number, ValidationError))
}

pub fn new() -> NumberField {
  from_string("")
}

pub fn from_string(value: String) -> NumberField {
  NumberField(value, number.from_string(value))
}

pub fn to_number(self: NumberField) -> Result(Number, ValidationError) {
  self.validated
}
