mod diff;
mod display;
mod io;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "diffy")]
#[command(about = "A fast diff checker written in Rust")]
#[command(version = concat!("v", env!("CARGO_PKG_VERSION")))]
struct Args {
    before: String,
    after: String,

    #[arg(short, long, default_value = "lcs")]
    algorithm: String,

    #[arg(short = 'n', long)]
    line_numbers: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let before = io::file_reader::read_file(&args.before)?;
    let after = io::file_reader::read_file(&args.after)?;

    let algorithm = diff::get_algorithm(&args.algorithm);

    let changes = algorithm.compute(&before, &after);

    let mut display = display::TerminalDisplay::new();
    if args.line_numbers {
        display = display.with_line_numbers();
    }
    display.display(&changes);

    Ok(())
}
