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
    pub log: Vec<String>,
}

impl Metrics {
    fn get_removed_blocks_cnt(&self) -> usize {
        self.marks.iter().fold(0, |acc, v| acc + v.lines.len())
    }

    fn get_removed_lines_cnt(&self) -> usize {
        self.marks.iter().fold(0, |acc1, v| {
            acc1 + v.lines.iter().fold(0, |acc2, &(b, e)| acc2 + e - b)
        })
    }

    pub fn fmt_summary_metrics(&self) -> String {
        let s1 = format!("Summary Results\n");
        let s2 = format!("  Total files:    {:5}\n", self.total_files);
        let s3 = format!("  Affected files: {:5}\n", self.marks.len());
        let s4 = format!("  Blocks removed: {:5}\n", self.get_removed_blocks_cnt());
        let s5 = format!("  Lines removed:  {:5}\n", self.get_removed_lines_cnt());
        s1 + &s2 + &s3 + &s4 + &s5
    }
}
