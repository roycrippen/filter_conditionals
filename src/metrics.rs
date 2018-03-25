use std::collections::HashSet;

#[derive(Default)]
pub struct Metrics {
    pub total_files: usize,
    pub removed_blocks: usize,
    pub affected_files: HashSet<String>,
    pub removed_lines: Vec<String>,
}

impl Metrics {
    pub fn print_summary_metrics(&self) {
        let removed_lines = self.removed_lines.len() - self.removed_blocks * 2;
        println!("\nSummary Results");
        println!("  Total files:    {:5}", self.total_files);
        println!("  Affected files: {:5}", self.affected_files.len());
        println!("  Blocks removed: {:5}", self.removed_blocks);
        println!("  Lines removed:  {:5}", removed_lines);
    }
}
