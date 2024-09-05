#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Number {
    Valid(usize),
    Invalid(String),
}

impl Number {
    pub fn is_valid_small(&self) -> bool {
        self.is_valid_using(SMALLS)
    }

    pub fn is_valid_large(&self) -> bool {
        self.is_valid_using(LARGES)
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_small() || self.is_valid_large()
    }

    fn is_valid_using(&self, valids: &[usize]) -> bool {
        match self {
            Number::Valid(value) => in_range_using(*value, valids),
            Number::Invalid(_) => false,
        }
    }
}

const SMALLS: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const LARGES: &[usize] = &[25, 50, 75, 100];

fn in_small_range(value: usize) -> bool {
    in_range_using(value, SMALLS)
}

fn in_large_range(value: usize) -> bool {
    in_range_using(value, LARGES)
}

fn in_range(value: usize) -> bool {
    in_small_range(value) || in_large_range(value)
}

fn in_range_using(value: usize, valids: &[usize]) -> bool {
    valids.contains(&value)
}

impl From<String> for Number {
    fn from(value: String) -> Self {
        let mut result = Number::Invalid(value.clone());

        if let Ok(number) = value.parse::<usize>() {
            if in_range(number) {
                result = Number::Valid(number)
            } else {
                result = Number::Invalid(value)
            }
        }

        result
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Invalid("".into())
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Number::Valid(value) => value.to_string(),
                Number::Invalid(value) => value.clone(),
            }
        )
    }
}
