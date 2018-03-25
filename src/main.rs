mod search;
mod metrics;
mod lib;

use search::Search;
use metrics::Metrics;
use lib::{get_file_list, is_affirmative, is_def_start, print_vec, read_file};

const MARKER: &'static str =
    "=========================================================================================\n";

fn main() {
    let mut metrics: Metrics = Default::default();
    let search: Search = Default::default();
    println!("{}", search);

    // let files = get_file_list("/home/crippenre/dev/booster/OBV2_4_SC");
    let files = get_file_list("/home/roy/dev/cpp/cppcheck");
    metrics.total_files = files.len();

    // search.debug_print_target_ifs(&files);
    // search.debug_print_unique_target_ifs(&files);

    // need to manually take care of nested targets
    if search.debug_print_nested_target_ifs(&files) {
        return ();
    }

    for (full_file, _file) in files {
        let mut ss = read_file(&full_file);
        let mut target_line = false;
        for mut s in ss {
            if is_def_start(&s) && search.contains_target(&s) {
                if is_affirmative(&s) && search.contains_target_usage(&s) {
                    target_line = true;
                    metrics.affected_files.insert(full_file.clone());
                    metrics.removed_blocks += 1;
                    metrics.removed_lines.push(full_file.clone());
                } else {
                }
            }

            if target_line {
                metrics.removed_lines.push(s.clone());
                // let s_ = "// ".to_string() + &s;
                if s.starts_with("#endif") || s.starts_with("#else") {
                    target_line = false;
                    metrics.removed_lines.push(MARKER.to_string());
                }
            }
        }
    }
    metrics.print_summary_metrics();

    // print_vec(&metrics.removed_lines);
}
