use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Mark {
    pub file: String,
    pub lines: Vec<(usize, usize)>,
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default)]
pub struct Metrics {
    pub total_files: usize,
    pub marks: Vec<Mark>,
}

impl Metrics {
    fn get_removed_blocks_cnt(&self) -> usize {
        self.marks
            .iter()
            .fold(0, |acc, v| {
                acc + v.lines.len()
            })
    } 

    fn get_removed_lines_cnt(&self) -> usize {
        self.marks
            .iter()
            .fold(0, |acc1, v| {
                acc1 + v.lines.iter().fold(0, |acc2, &(b, e)| { acc2 + e - b })
            })
    } 

    pub fn print_summary_metrics(&self) {
        println!("\nSummary Results");
        println!("  Total files:    {:5}", self.total_files);
        println!("  Affected files: {:5}", self.marks.len());
        println!("  Blocks removed: {:5}", self.get_removed_blocks_cnt());
        println!("  Lines removed:  {:5}", self.get_removed_lines_cnt());
    }
}
