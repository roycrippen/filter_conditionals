mod search;
mod metrics;
mod lib;

use search::Search;
use metrics::Metrics;
use lib::*;

const MARKER: &'static str =
    "=========================================================================================\n";

fn main() {
    let mut metrics: Metrics = Default::default();
    let search: Search = Default::default();
    println!("{}", search);

    let files = get_file_list("/home/crippenre/dev/booster/OBV2_4_SC");
    metrics.total_files = files.len();

    search.debug_print_unique_target_ifs(&files);

    for (full_file, _file) in files.clone() {
        let mut ss = read_file(&full_file);

        // process nested
        for i in 0..10 {
            let nested = search.get_nested_affirmative_target(&ss, &full_file);
            if nested.start_line > 0 {
                clean_nested(&mut ss, &nested);
                print_clean_nested(&ss, &nested, false);
            } else {
                break;
            }
        }

        let mut target_line = false;

        // process affirmative start_defs
        let mut nested: Nested = Default::default();
        for (i, s) in ss.clone().into_iter().enumerate() {
            let s = s.trim();
            if is_start(s) {
                if is_affirmative(s) && search.contains_target_usage(s) {
                    nested.start_line = i + 1;
                    target_line = true;
                    metrics.affected_files.insert(full_file.clone());
                    metrics.removed_blocks += 1;
                    metrics.removed_lines.push(full_file.clone());
                } else {
                }
            }

            if target_line {
                metrics.removed_lines.push(s.clone().to_string());
                if is_end(s) {
                    target_line = false;
                    metrics.removed_lines.push(MARKER.to_string());
                    nested.end_line = i + 1;
                }
            }
        }
        clean_nested(&mut ss, &nested);

        // process not affirmative
    }
    metrics.print_summary_metrics();

    // print_vec(&metrics.removed_lines);
}
