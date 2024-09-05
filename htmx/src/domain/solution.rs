#[derive(Clone, Debug, Default)]
pub enum Solution {
    #[default]
    Unrequested,
    Pending,
    Solved,
    Failed(String),
}
