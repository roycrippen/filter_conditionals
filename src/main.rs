#[macro_use]
extern crate clap;
use clap::App;

mod search;
mod metrics;

use search::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source_path = matches.value_of("INPUT").unwrap();

    println!("{:?}", matches);

    if matches.is_present("build") {
        let search: Search = Default::default();
        let files = get_file_list(source_path, is_source);
        search.write_unique_target_ifs(&files);
        println!("Unique target files built; inspect and modify them as necessary.");
        println!("Files created:\n   affirmative_targets.txt\n   non_affirmative_targets.txt");
    } else if matches.is_present("explore") {
        let files = get_file_list(source_path, is_source);
        let metrics = process_source(&files);
        write_log(&metrics.log);
        println!("No source file were modified.\n");
        println!("{}", metrics.fmt_summary_metrics());
    } else if matches.is_present("remove") {
        let files = get_file_list(source_path, is_source);
        let metrics = process_source(&files);
        undo(source_path);
        backup_files(&files, "original");
        write_new_source_files(&metrics);
        write_log(&metrics.log);
        println!("SOURCE WERE FILES MODIFIED!\n");
        println!("{}", metrics.fmt_summary_metrics());
    } else if matches.is_present("undo") {
        println!("Source files restored: {}\n", undo(source_path));
    } else {
        println!("No valid option selected.\n-h for help");
    }
}
