extern crate walkdir;

use walkdir::{DirEntry, WalkDir};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::fmt::Display;

const MARKER: &'static str = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz\n";

struct Metrics {
    total_files: usize,
    removed_blocks: usize,
    affected_files: HashSet<String>,
    removed_lines: Vec<String>,
}

#[derive(Debug)]
enum BlockTerminator {
    Endif,
    ElseOrEndif,
}

use BlockTerminator::*;

fn get_file_list(path: &str) -> Vec<(String, String)> {
    WalkDir::new(path)
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| is_source(e))
        .map(|e| {
            (
                e.path().display().to_string(),
                e.file_name().to_str().unwrap().to_string(),
            )
        })
        .collect()
}

fn is_source(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".cc") || s.ends_with(".c") || s.ends_with(".hh") || s.ends_with(".h"))
        .unwrap_or(false)
}

fn read_file(path: &String) -> Vec<String> {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|s| s.to_string()).collect()
}

fn is_start_def(s: &String) -> bool {
    s.starts_with("#if") || s.starts_with("#ifdef") || s.starts_with("#ifndef")
}

fn contains_target(s: &String) -> bool {
    s.contains("NRTSIM") || s.contains("RTCLSIM")
}

fn is_affirmative(s: &String) -> bool {
    !s.contains("!")
}

fn get_qualified_file(s: String) -> String {
    let (_, qualified_file) = s.split_at(s.find("OBV2_4_SC").unwrap_or(0));
    qualified_file.to_string()
}

#[allow(dead_code)]
fn debug_print_unique_target_ifs(files: &Vec<(String, String)>) {
    let mut xs = vec![];
    let mut ys = vec![];
    for &(ref fully_qualified_file, ref _file) in files {
        let ss = read_file(&fully_qualified_file);
        for s in ss {
            if is_start_def(&s) && contains_target(&s) {
                if is_affirmative(&s) {
                    xs.push(s);
                } else {
                    ys.push(s);
                }
            }
        }
    }
    xs.sort();
    xs.dedup();
    for x in xs.iter() {
        println!("affirmative if: {:50}", x)
    }
    println!("");
    ys.sort();
    ys.dedup();
    for y in ys.iter() {
        println!("exclusive if: {:50}", y)
    }
}

fn pred_pass_1(s: &str) -> bool {
    s.starts_with("#if ( NRTSIM )") || s.starts_with("#if (NRTSIM)")
        || s.starts_with("#ifdef NRTSIM") || s.starts_with("#if ( NRTSIM || RTCLSIM )")
        || s.starts_with("#if (NRTSIM || RTCLSIM)")
}

fn process_file(
    ss: &mut Vec<String>,
    metrics: &mut Metrics,
    file: &String,
    predicate: fn(s: &str) -> bool,
    terminator: BlockTerminator,
) {
    let mut target_line = false;
    for mut s in ss {
        if is_start_def(&s) && contains_target(&s) {
            if is_affirmative(&s) && predicate(&s) {
                target_line = true;
                metrics.affected_files.insert(file.clone());
                metrics.removed_blocks += 1;
                metrics.removed_lines.push(get_qualified_file(file.clone()));
            } else {
            }
        }

        if target_line {
            metrics.removed_lines.push(s.clone());
            let s_ = "// ".to_string() + s;
            match terminator {
                BlockTerminator::ElseOrEndif => {
                    if s.starts_with("#endif") || s.starts_with("#else") {
                        target_line = false;
                        metrics.removed_lines.push(MARKER.to_string());
                    }
                }
                BlockTerminator::Endif => (),
            }
        }
    }
}

fn print_summary_metric(m: &Metrics) {
    let removed_lines = m.removed_lines.len() - m.removed_blocks * 2;
    println!("Total files:    {:5}", m.total_files);
    println!("Affected files: {:5}", m.affected_files.len());
    println!("Blocks removed: {:5}", m.removed_blocks);
    println!("Lines removed:  {:5}", removed_lines);
}

#[allow(dead_code)]
fn print_vec<T: Display>(xs: &Vec<T>) {
    xs.iter().for_each(|x| println!("{}", x))
}

fn main() {
    let mut metrics: Metrics = Metrics {
        total_files: 0,
        affected_files: HashSet::new(),
        removed_blocks: 0,
        removed_lines: vec![],
    };

    let files = get_file_list("/home/crippenre/dev/booster/OBV2_4_SC");
    // debug_print_unique_target_ifs(&files);
    metrics.total_files = files.len();
    for (full_file, _file) in files {
        if _file != "OscMemory.hh" {
            continue;
        }
        let mut ss = read_file(&full_file);
        process_file(&mut ss, &mut metrics, &full_file, pred_pass_1, ElseOrEndif);
        print_vec(&ss);
    }
    print_summary_metric(&metrics);
    // print_vec(&metrics.removed_lines);
}
