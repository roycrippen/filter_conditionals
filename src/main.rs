#[macro_use]
extern crate clap;
use clap::App;

mod search;
mod metric;

use search::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source_path = matches.value_of("INPUT").unwrap();

    if matches.is_present("build") {
        let search: Search = Default::default();
        let files = get_file_list(source_path, is_source);
        search.write_unique_target_ifs(&files);
        println!("Unique target files built; inspect and modify them as necessary.");
        println!("Files created:\n   affirmative_targets.txt\n   non_affirmative_targets.txt");
    } else if matches.is_present("explore") {
        let files = get_file_list(source_path, is_source);
        let metric = process_source(&files);
        write_log(&metric.log);
        println!("No source file were modified.\n");
        println!("{}", metric.fmt_summary_metric());
    } else if matches.is_present("remove") {
        let files = get_file_list(source_path, is_source);
        let metric = process_source(&files);
        undo(source_path);
        let target_files = metric.marks.iter().map(|mark| mark.file.clone()).collect();
        backup_files(&target_files, "original");
        write_new_source_files(&metric.marks);
        write_log(&metric.log);
        println!("SOURCE WERE FILES MODIFIED!\n");
        println!("{}", metric.fmt_summary_metric());
    } else if matches.is_present("undo") {
        println!("Source files restored: {}\n", undo(source_path));
    } else {
        println!("No valid option selected.\n-h for help");
    }
}
