mod search;
mod metrics;
mod lib;

use search::Search;
use metrics::*;
use lib::*;

const SOURCE: &'static str = "/home/crippenre/dev/booster/OBV2_4_SC";

fn main() {
    let mut metrics: Metrics = Default::default();
    let mut marks: Vec<Mark> = Vec::new();
    let search: Search = Default::default();
    println!("{}", search);

    let files = get_file_list(SOURCE);
    metrics.total_files = files.len();

    // search.debug_print_unique_target_ifs(&files);

    for (full_file, _file) in files.clone() {
        let mut mark: Mark = Default::default();
        mark.file = full_file.to_string();

        let mut ss = read_file(&full_file);
        for (i, s) in ss.iter().enumerate() {
            if is_start(&s) {
                if search.affirmative_targets.contains(s) {
                    let end_line = find_conditional_end_line(&ss, i, true);
                    mark.lines.push((i, end_line));
                // println!("{}", full_file);
                // print_vec_range(&ss, i, end_line);
                // println!("\n")
                } else {
                    // non-affirmatives
                }
            }
        }
        if mark.lines.len() > 0 {
            marks.push(mark);
        }
    }
    metrics.marks = marks;

    metrics.print_summary_metrics();
}
