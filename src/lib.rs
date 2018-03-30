#![allow(dead_code)]
extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir};
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;

pub fn is_start(s: &str) -> bool {
    s.starts_with("#")
        && (s.starts_with("#if") || s.starts_with("#ifdef") || s.starts_with("#elif")
            || s.starts_with("#ifndef"))
}

pub fn is_end(s: &str, else_end: bool) -> bool {
    s.starts_with("#endif") || s.starts_with("#elif") || (s.starts_with("#else") && else_end)
}

pub fn is_affirmative(s: &str) -> bool {
    !s.contains("!") && !s.starts_with("#ifndef")
}

pub fn read_file(path: &str) -> Vec<String> {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|s| s.to_string()).collect()
}

pub fn get_file_list(path: &str) -> Vec<(String, String)> {
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

pub fn find_conditional_end_line(ss: &Vec<String>, start_line: usize, else_end: bool) -> usize {
    let mut current_line = start_line;
    let mut next_line = current_line + 1;
    let mut it = ss.iter().skip(next_line);

    loop {
        current_line += 1;
        match it.next() {
            Some(s) => {
                if next_line > current_line {
                    continue;
                }
                if is_start(s) {
                    next_line = find_conditional_end_line(&ss, current_line, false) + 1;
                }
                if is_end(s, else_end) {
                    return current_line;
                }
            }
            None => break,
        }
    }
    0
}
