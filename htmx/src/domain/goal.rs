#[derive(Clone, Debug, Default)]
pub enum Goal {
    #[default]
    Auto,
    Target(usize),
    Invalid(String),
}

impl Goal {
    pub fn is_valid(&self) -> bool {
        match self {
            Goal::Auto => true,
            Goal::Target(target) => in_range(*target),
            _ => false, 
        }
    }

    pub fn reveal(&mut self) {
        match self {
            Goal::Auto => *self = Goal::Target(42),
            _ => {},
        }
    }
}

fn in_range(value: usize) -> bool {
    (100..=999).contains(&value)
}

impl From<String> for Goal {
    fn from(value: String) -> Self {
        let mut result = Goal::Invalid(value.clone());

        if let Ok(target) = value.parse::<usize>() {
            if in_range(target) {
                result = Goal::Target(target)
            } else if value.is_empty() {
                result = Goal::Auto                
            }
        }

        result
    }
}

impl std::fmt::Display for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Goal::Auto => "".to_string(),
                Goal::Target(number) => number.to_string(),
                Goal::Invalid(value) => value.clone(),
            }
        )
    }
}
