#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    Added(String),
    Deleted(String),
    Unchanged(String),
}

pub trait DiffAlgorithm {
    fn compute(&self, before: &[String], after: &[String]) -> Vec<Change>;
    fn name(&self) -> &str;
}
