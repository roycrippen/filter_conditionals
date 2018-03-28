#![allow(dead_code)]
extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir};
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Mark {
    pub conditional: String,
    pub start_line: usize,
    pub end_line: usize,
    pub file: String,
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn is_start(s: &str) -> bool {
    s.starts_with("#")
        && (s.starts_with("#if") || s.starts_with("#ifdef") || s.starts_with("#elif"))
}

pub fn is_end(s: &str) -> bool {
    s.starts_with("#endif") || s.starts_with("#elif") || s.starts_with("#else")
}

pub fn is_affirmative(s: &str) -> bool {
    !s.contains("!") && !s.starts_with("#ifndef")
}

pub fn read_file(path: &String) -> Vec<String> {
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

pub fn comment_lines(ss: &mut Vec<String>, mark: &Mark) {
    if mark.start_line > 0 && mark.end_line > 0 {
        (mark.start_line - 1..mark.end_line).for_each(|i| ss[i] = format!("// {}", ss[i]));
    }
}

pub fn print_commented_lines(ss: &Vec<String>, mark: &Mark, detailed: bool) {
    if mark.start_line > 0 && mark.end_line > 0 {
        println!("{}", mark);
        if detailed {
            print_vec_range(&ss, mark.start_line - 1, mark.end_line - 1);
            println!("");
        }
    }
}
