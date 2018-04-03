#[macro_use]
extern crate clap;
use clap::App;

mod search;
mod metrics;

use search::{get_file_list, process_source, write_log, Search};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source = matches.value_of("INPUT").unwrap();

    if matches.is_present("build") {
        let search: Search = Default::default();
        let files = get_file_list(source);
        search.write_unique_target_ifs(&files);
        println!("Unique target files built; inspect and modify them as necessary.");
        println!("Files created:\n   affirmative_targets.txt\n   non_affirmative_targets.txt");
    } else if matches.is_present("remove") {
        let metrics = process_source(source);
        write_log(&metrics.log);
        println!("{}", metrics.fmt_summary_metrics());
    } else {
        println!("No valid option selected.\n-h for help");
    }
}
