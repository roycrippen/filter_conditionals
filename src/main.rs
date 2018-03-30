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

    // search.debug_print_unique_target_ifs(&files);

    for (full_file, _file) in files.clone() {
        if full_file == "/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/VectorOperations.hh"
        {
            println!("{}", full_file);
            let mut ss = read_file(&full_file);
            print_vec_range(&ss, 14, 43);
            // let _aaa = find_conditional_end_line(&ss, 0);
            // println!("{}", _aaa)
        }
    }
    metrics.print_summary_metrics();
}
