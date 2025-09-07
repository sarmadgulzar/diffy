use anyhow::Result;
use std::fs;

pub fn read_file(file_name: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(file_name)?;
    Ok(content.lines().map(String::from).collect())
}
