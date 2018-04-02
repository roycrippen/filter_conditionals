mod search;
mod metrics;
mod utils;

use search::Search;
use metrics::*;
use utils::*;

const SOURCE: &'static str = "/home/crippenre/dev/booster/OBV2_4_SC";

fn main() {
    let search: Search = Default::default();
    let mut metrics: Metrics = Default::default();
    metrics.log.push(format!("{}", search));

    let files = get_file_list(SOURCE);
    metrics.total_files = files.len();

    // search.debug_print_unique_target_ifs(&files);

    for (full_file, _file) in files.clone() {
        let mut mark: Mark = Default::default();
        mark.file = full_file.to_string();

        let mut ss = read_file(&full_file);
        let mut start_line = 0;
        let mut end_line = 0;
        for (i, s) in ss.iter().enumerate() {
            if start_line > i || end_line > i {
                continue;
            }
            if is_start(&s) {
                if search.affirmative_targets.contains(s) {
                    end_line = find_conditional_end_line(&ss, i, true);
                    if end_line == 0 {
                        continue;
                    }
                    mark.lines.push((i, end_line));
                    metrics.log.push(format!("file:        {}", full_file));
                    metrics.log.push(format!("Conditional: {}", s));
                    let mut removed = add_line_nums_vec_range(&ss, i, end_line);
                    metrics.log.append(&mut removed);
                    metrics.log.push("\n".to_string());
                } else if search.non_affirmative_targets.contains(s) {
                    start_line = find_else_start_line(&ss, i);
                    if start_line == 0 {
                        continue;
                    }
                    let end_line = find_conditional_end_line(&ss, start_line, false);
                    if end_line == 0 {
                        continue;
                    }
                    mark.lines.push((start_line, end_line));
                    metrics.log.push(format!("file:        {}", full_file));
                    metrics.log.push(format!("Conditional: {}", s));
                    let mut removed = add_line_nums_vec_range(&ss, start_line, end_line);
                    metrics.log.append(&mut removed);
                    metrics.log.push("\n".to_string());
                }
            }
        }
        if mark.lines.len() > 0 {
            metrics.marks.push(mark);
        }
    }
    let summary = metrics.fmt_summary_metrics();
    metrics.log.push(summary.clone());
    write_log(&metrics.log);
    println!("{}", summary);
}
