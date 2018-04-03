#![allow(dead_code)]
extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir};
use std::io::prelude::{Read, Write};
use std::fs::File;
use std::fmt::Display;
use std::collections::HashSet;
use std::fmt;

use metrics::*;

pub struct Search {
    pub targets: HashSet<String>,
    pub affirmative_targets: HashSet<String>,
    pub non_affirmative_targets: HashSet<String>,
}

impl<'a> Search {
    pub fn contains_target(&self, s: &str) -> bool {
        self.targets.iter().any(|k| s.contains(k))
    }

    pub fn debug_print_unique_target_ifs(&self, files: &Vec<(String, String)>) {
        let mut xs = vec![];
        let mut ys = vec![];
        for &(ref fully_qualified_file, ref _file) in files {
            let ss = read_file(&fully_qualified_file);
            for s in ss {
                if is_start(&s) && self.contains_target(&s) {
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
        println!("Affirmative targets:");
        for x in xs.iter() {
            println!("{}", x.trim())
        }
        println!("");

        ys.sort();
        ys.dedup();
        println!("Non-affirmative targets:");
        for y in ys.iter() {
            println!("{}", y.trim())
        }
        println!("")
    }
}

impl<'a> Default for Search {
    fn default() -> Search {
        let read = |s: &str| -> HashSet<String> {
            read_file(s)
                .into_iter()
                .filter(|s| !s.starts_with("//"))
                .collect()
        };

        let targets = read("targets.txt");
        let affirmative_targets = read("affirmative_targets.txt");
        let non_affirmative_targets = read("non_affirmative_targets.txt");
        Search {
            targets: targets,
            affirmative_targets: affirmative_targets,
            non_affirmative_targets: non_affirmative_targets,
        }
    }
}

impl<'a> fmt::Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_str = |set: &HashSet<String>| -> String {
            set.iter().fold("\n  [ ".to_string(), |mut acc, k| {
                acc.push_str("\n    ");
                acc.push_str(k);
                acc
            })
        };

        let s_targets = format!("Target identifiers: {}\n  ]\n", fmt_str(&self.targets));
        let s_affirmative_targets = format!(
            "\nAffirmative targets: {}\n  ]\n",
            fmt_str(&self.affirmative_targets)
        );
        let s_non_affirmative_targets = format!(
            "\nNon-affirmative targets: {}\n  ]\n",
            fmt_str(&self.non_affirmative_targets)
        );

        write!(
            f,
            "{}{}{}",
            s_targets, s_affirmative_targets, s_non_affirmative_targets
        )
    }
}

pub fn process_source(source: &str) -> Metrics {
    let search: Search = Default::default();
    let mut metrics: Metrics = Default::default();
    metrics.log.push(format!("{}", search));

    let files = get_file_list(source);
    metrics.total_files = files.len();

    // search.debug_print_unique_target_ifs(&files);

    for (full_file, _file) in files.clone() {
        let mut mark: Mark = Default::default();
        mark.file = full_file.to_string();

        let mut ss = read_file(&full_file);
        let mut start_line = 0;
        let mut end_line = 0;
        for (i, s) in ss.iter().enumerate() {
            let process_lines =
                |mark: &mut Mark, metrics: &mut Metrics, start_line: usize, end_line: usize| {
                    mark.lines.push((start_line, end_line));
                    metrics.log.push(format!("file:        {}", full_file));
                    metrics.log.push(format!("Conditional: {}", s));
                    let mut removed = add_line_nums_vec_range(&ss, start_line, end_line);
                    metrics.log.append(&mut removed);
                    metrics.log.push("\n".to_string());
                };

            if start_line > i || end_line > i {
                continue;
            }
            if is_start(&s) {
                if search.affirmative_targets.contains(s) {
                    end_line = find_conditional_end_line(&ss, i, true);
                    if end_line == 0 {
                        continue;
                    }
                    process_lines(&mut mark, &mut metrics, i, end_line);
                } else if search.non_affirmative_targets.contains(s) {
                    start_line = find_else_start_line(&ss, i);
                    if start_line == 0 {
                        continue;
                    }
                    let end_line = find_conditional_end_line(&ss, start_line, false);
                    if end_line == 0 {
                        continue;
                    }
                    process_lines(&mut mark, &mut metrics, start_line, end_line);
                }
            }
        }
        if mark.lines.len() > 0 {
            metrics.marks.push(mark);
        }
    }
    let summary = metrics.fmt_summary_metrics();
    metrics.log.push(summary.clone());
    metrics
}

fn is_start(s: &str) -> bool {
    s.starts_with("#")
        && (s.starts_with("#if") || s.starts_with("#ifdef") || s.starts_with("#elif")
            || s.starts_with("#ifndef"))
}

fn is_end(s: &str, else_end: bool) -> bool {
    s.starts_with("#endif") || s.starts_with("#elif") || (s.starts_with("#else") && else_end)
}

fn is_affirmative(s: &str) -> bool {
    !s.contains("!") && !s.starts_with("#ifndef")
}

fn read_file(path: &str) -> Vec<String> {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|s| s.to_string()).collect()
}

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

fn find_conditional_end_line(ss: &Vec<String>, start_line: usize, else_end: bool) -> usize {
    let mut current_line = start_line;
    let mut next_line = current_line + 1;
    let mut it = ss.iter().skip(next_line);

    loop {
        current_line += 1;
        match it.next() {
            Some(s) => {
                // skip line nested block had been found
                if next_line > current_line {
                    continue;
                }

                // skip nested conditionals
                if is_start(s) {
                    next_line = find_conditional_end_line(&ss, current_line, false) + 1;
                }

                // done id end is found
                if is_end(s, else_end) {
                    return current_line;
                }
            }
            None => break,
        }
    }

    // not found
    0
}

fn find_else_start_line(ss: &Vec<String>, start_line: usize) -> usize {
    let mut current_line = start_line;
    let mut next_line = current_line + 1;
    let mut it = ss.iter().skip(next_line);

    loop {
        current_line += 1;
        match it.next() {
            Some(s) => {
                // skip line nested block had been found
                if next_line > current_line {
                    continue;
                }

                // skip nested conditionals
                if is_start(s) {
                    next_line = find_conditional_end_line(&ss, current_line, false) + 1;
                }

                // done id end is found
                if is_end(s, false) {
                    return 0;
                }

                // done if else found
                if s.starts_with("#else") {
                    return current_line;
                }
            }
            None => break,
        }
    }

    // not found
    0
}

pub fn write_log(log: &Vec<String>) {
    let mut buffer = File::create("results.txt").unwrap();
    log.iter().for_each(|s| write!(buffer, "{}\n", s).unwrap());
}

fn is_source(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.ends_with(".cc") || s.ends_with(".c") || s.ends_with(".hh") || s.ends_with(".h")
                || s.ends_with(".cpp") || s.ends_with(".cxx")
        })
        .unwrap_or(false)
}

pub fn print_vec<T: Display>(xs: &Vec<T>) {
    xs.iter().for_each(|x| println!("{}", x))
}

pub fn print_vec_range<T: Display>(xs: &Vec<T>, start: usize, end: usize) {
    (start..end + 1).for_each(|i| println!("{:4} {}", i + 1, xs[i]))
}

fn add_line_nums_vec_range<T: Display>(xs: &Vec<T>, start: usize, end: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    (start..end + 1).for_each(|i| res.push(format!("{:4} {}", i + 1, xs[i])));
    res
}
