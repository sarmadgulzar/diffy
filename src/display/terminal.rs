use colored::*;

use crate::diff::types::Change;

pub struct TerminalDisplay {
    show_line_numbers: bool,
    context_lines: Option<usize>,
}

impl TerminalDisplay {
    pub fn new() -> Self {
        Self {
            show_line_numbers: false,
            context_lines: None,
        }
    }

    pub fn with_line_numbers(mut self) -> Self {
        self.show_line_numbers = true;
        self
    }

    pub fn display(&self, changes: &[Change]) {
        let mut before_line = 1;
        let mut after_line = 1;

        for change in changes {
            match change {
                Change::Unchanged(line) => {
                    self.print_unchanged(line, before_line, after_line);
                    before_line += 1;
                    after_line += 1;
                }
                Change::Added(line) => {
                    self.print_added(line, after_line);
                    after_line += 1;
                }
                Change::Deleted(line) => {
                    self.print_deleted(line, before_line);
                    before_line += 1;
                }
            }
        }
    }

    fn print_unchanged(&self, line: &str, before_num: usize, after_num: usize) {
        if self.show_line_numbers {
            print!("{:4} {:4} ", before_num, after_num);
        }
        println!("  {}", line);
    }

    fn print_added(&self, line: &str, after_num: usize) {
        if self.show_line_numbers {
            print!("     {:4} ", after_num);
        }
        println!("{} {}", "+".green().bold(), line.green());
    }

    fn print_deleted(&self, line: &str, before_num: usize) {
        if self.show_line_numbers {
            print!("{:4}      ", before_num);
        }
        println!("{} {}", "-".red().bold(), line.red());
    }
}
