# diffy 🔍
A fast diff checker written in Rust

## Installation

### From GitHub (Recommended)
```bash
# Install directly from GitHub
cargo install --git https://github.com/sarmadgulzar/diffy

# Install from a specific branch or tag
cargo install --git https://github.com/sarmadgulzar/diffy --branch main
cargo install --git https://github.com/sarmadgulzar/diffy --tag v0.1.0
```

### From Source
```bash
# Clone and install locally
git clone https://github.com/sarmadgulzar/diffy.git
cd diffy
cargo install --path .

# Or just build and run directly
cargo build --release
./target/release/diffy file1.txt file2.txt
```

## Usage

```bash
# Basic usage
diffy before.txt after.txt

# With line numbers
diffy -n before.txt after.txt
diffy --line-numbers before.txt after.txt

# Specify algorithm (currently only LCS available)
diffy --algorithm lcs before.txt after.txt
```

## Examples

### Quick Test
Create these test files to try out diffy:

**Create `test_files/version1.rs`:**
```rust
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
```

**Create `test_files/version2.rs`:**
```rust
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
```

**Run diffy:**
```bash
diffy test_files/version1.rs test_files/version2.rs
```

**Output:**
```diff
- fn process_data(input: &str) -> String {
+ use std::error::Error;
+
+ fn process_data(input: &str) -> Result<String, Box<dyn Error>> {
      let result = input.to_uppercase();
-     println!("Processing: {}", input);
-     return result;
+     log::info!("Processing: {}", input);
+     Ok(result)
  }

- fn main() {
+ fn main() -> Result<(), Box<dyn Error>> {
      let data = "hello world";
-     let output = process_data(data);
-     print!("{}", output);
+     let output = process_data(data)?;
+     println!("{}", output);
+     Ok(())
  }
```

**With line numbers:**
```bash
diffy -n test_files/version1.rs test_files/version2.rs
```

**Output:**
```diff
1      - fn process_data(input: &str) -> String {
     1 + use std::error::Error;
     2 +
     3 + fn process_data(input: &str) -> Result<String, Box<dyn Error>> {
2    4       let result = input.to_uppercase();
3      -     println!("Processing: {}", input);
4      -     return result;
     5 +     log::info!("Processing: {}", input);
     6 +     Ok(result)
5    7   }
6    8
7      - fn main() {
     9 + fn main() -> Result<(), Box<dyn Error>> {
8   10       let data = "hello world";
9      -     let output = process_data(data);
10      -     print!("{}", output);
    11 +     let output = process_data(data)?;
    12 +     println!("{}", output);
    13 +     Ok(())
11   14   }
```

## Features

- 🚀 **Fast** - Efficient LCS (Longest Common Subsequence) algorithm implementation
- 🎨 **Colored output** - Red for deletions, green for additions
- 🔢 **Line numbers** - Optional line number display for both files
- 📁 **File-based** - Compare any two text files
- 🦀 **Written in Rust** - Memory safe and blazingly fast™

## Algorithms

Currently supports:
- **LCS (Longest Common Subsequence)** - Classic diff algorithm that finds the longest sequence of lines that appear in both files

Future algorithms planned:
- Myers' diff algorithm
- Patience diff
- Histogram diff

## License

This project is released under The Unlicense - see the [LICENSE](LICENSE) file for details.

---

*Built with 🦀 and ❤️*
