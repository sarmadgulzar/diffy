use std::error::Error;

fn process_data(input: &str) -> Result<String, Box<dyn Error>> {
    let result = input.to_uppercase();
    log::info!("Processing: {}", input);
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = "hello world";
    let output = process_data(data)?;
    println!("{}", output);
    Ok(())
}
