#[derive(Clone, Debug)]
pub struct Result {
    pub value: Option<i32>,
    pub node_count: i32,
    pub description: String,
}

impl Result {
    pub fn new(value: Option<i32>, node_count: i32, description: &str) -> Result {
        Result {
            value,
            node_count,
            description: description.to_string(),
        }
    }
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
