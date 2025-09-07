pub mod algorithms;
pub mod types;

use types::DiffAlgorithm;

/// Factory function to get the default algorithm
pub fn get_default_algorithm() -> Box<dyn DiffAlgorithm> {
    Box::new(algorithms::lcs::LcsDiff::new())
}

pub fn get_algorithm(name: &str) -> Box<dyn DiffAlgorithm> {
    match name.to_lowercase().as_str() {
        "lcs" => Box::new(algorithms::lcs::LcsDiff::new()),
        _ => get_default_algorithm(),
    }
}
