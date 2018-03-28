mod search;
mod metrics;
mod lib;

use search::Search;
use metrics::Metrics;
use lib::*;

const SOURCE: &'static str = "/home/crippenre/dev/booster/OBV2_4_SC";

fn main() {
    let mut metrics: Metrics = Default::default();
    let search: Search = Default::default();
    println!("{}", search);

    let files = get_file_list(SOURCE);
    metrics.total_files = files.len();

    search.debug_print_unique_target_ifs(&files);

    for (full_file, _file) in files.clone() {
        let mut ss = read_file(&full_file);

        // process affirmatives
        search.process_nested_affirmatives(&mut ss, &full_file, true);
        search.process_affirmative(&mut ss, &mut metrics, &full_file, true)

        // process not affirmative
    }
    metrics.print_summary_metrics();
    // print_vec(&metrics.removed_lines);
}
