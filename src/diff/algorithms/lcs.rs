use std::vec;

use crate::diff::types::{Change, DiffAlgorithm};

pub struct LcsDiff;

impl DiffAlgorithm for LcsDiff {
    fn compute(&self, before: &[String], after: &[String]) -> Vec<Change> {
        let lcs_table = self.build_lcs_table(before, after);
        self.generate_changes(before, after, &lcs_table)
    }
    fn name(&self) -> &str {
        "LCS (Longest Common Subsequence"
    }
}

impl LcsDiff {
    pub fn new() -> Self {
        Self
    }

    fn build_lcs_table(&self, before: &[String], after: &[String]) -> Vec<Vec<usize>> {
        let m = before.len();
        let n = after.len();

        // Create the table with extra row/column for empty string case
        let mut table = vec![vec![0; n + 1]; m + 1];

        // Fill in the table
        for i in 1..=m {
            for j in 1..=n {
                if before[i - 1] == after[j - 1] {
                    // Lines match! Take diagonal value + 1
                    table[i][j] = table[i - 1][j - 1] + 1
                } else {
                    // Lines don't match, take the maximum from left or top
                    table[i][j] = table[i - 1][j].max(table[i][j - 1])
                }
            }
        }
        table
    }

    fn generate_changes(
        &self,
        before: &[String],
        after: &[String],
        table: &Vec<Vec<usize>>,
    ) -> Vec<Change> {
        let mut changes = Vec::new();
        let mut i = before.len();
        let mut j = after.len();

        // Backtrack through the table from bottom-right to top-left
        while i > 0 || j > 0 {
            // Case 1: We've processed all lines from 'after'
            if j == 0 {
                i -= 1;
                changes.push(Change::Deleted(before[i].clone()));
            }
            // Case 2: We've processed all lines from 'before'
            else if i == 0 {
                j -= 1;
                changes.push(Change::Added(after[j].clone()));
            }
            // Case 3: Lines match - it's unchanged
            else if before[i - 1] == after[j - 1] {
                i -= 1;
                j -= 1;
                changes.push(Change::Unchanged(before[i].clone()));
            }
            // Case 4: Lines don't match, check where we came from
            else if table[i - 1][j] > table[i][j - 1] {
                // Came from top, this line was deleted
                i -= 1;
                changes.push(Change::Deleted(before[i].clone()));
            } else {
                // Came from left, this line was deleted
                j -= 1;
                changes.push(Change::Added(after[j].clone()));
            }
        }
        changes.reverse(); // We built it backwards
        changes
    }
}
