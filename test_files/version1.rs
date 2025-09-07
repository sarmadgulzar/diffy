fn process_data(input: &str) -> String {
    let result = input.to_uppercase();
    println!("Processing: {}", input);
    return result;
}

fn main() {
    let data = "hello world";
    let output = process_data(data);
    print!("{}", output);
}
