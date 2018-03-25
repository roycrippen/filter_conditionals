#![allow(dead_code)]
extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir};
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;

pub fn is_def_start(s: &String) -> bool {
    s.starts_with("#if") || s.starts_with("#ifdef") || s.starts_with("#ifndef")
}

pub fn is_affirmative(s: &String) -> bool {
    !s.contains("!")
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

// #[derive(Debug)]
// enum BlockTerminator {
//     Endif,
//     ElseOrEndif,
// }

// use BlockTerminator::*;
