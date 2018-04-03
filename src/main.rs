mod search;
mod metrics;

use search::{process_source, write_log};

const SOURCE: &'static str = "/home/crippenre/dev/booster/OBV2_4_SC";

fn main() {
    let metrics = process_source(SOURCE);
    write_log(&metrics.log);
    println!("{}", metrics.fmt_summary_metrics());
}
