#![allow(dead_code)]
use std::collections::HashSet;
use std::fmt;

use lib::{is_affirmative, is_def_start, print_vec, read_file};

pub struct Search<'a> {
    pub targets: HashSet<&'a str>,
    pub target_usages: HashSet<&'a str>,
}

impl<'a> Search<'a> {
    pub fn contains_target(&self, s: &str) -> bool {
        self.targets.iter().any(|k| s.contains(k))
    }

    pub fn contains_target_usage(&self, s: &str) -> bool {
        self.target_usages.iter().any(|k| s.contains(k))
    }

    pub fn debug_print_unique_target_ifs(&self, files: &Vec<(String, String)>) {
        let mut xs = vec![];
        let mut ys = vec![];
        for &(ref fully_qualified_file, ref _file) in files {
            let ss = read_file(&fully_qualified_file);
            for s in ss {
                if is_def_start(&s) && self.contains_target(&s) {
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
            println!("not affirmative if: {:50}", y)
        }
    }

    pub fn debug_print_target_ifs(&self, files: &Vec<(String, String)>) {
        let mut xs = vec![];
        for &(ref fully_qualified_file, ref _file) in files {
            let ss = read_file(&fully_qualified_file);
            for s in ss {
                if is_def_start(&s) && self.contains_target(&s) {
                    xs.push(s);
                }
            }
        }
        xs.sort();
        print_vec(&xs);
    }

    // find nested target ifs
    // comments out the appropriate sections of these files by hand
    pub fn debug_print_nested_target_ifs(&self, files: &Vec<(String, String)>) -> bool {
        let mut target_line = false;
        let mut ns = Vec::new();
        for &(ref fully_qualified_file, ref _file) in files {
            let ss = read_file(&fully_qualified_file);
            for s in ss {
                if target_line {
                    if is_def_start(&s) {
                        ns.push(fully_qualified_file);
                    }

                    if s.starts_with("#endif") || s.starts_with("#else") {
                        target_line = false;
                    }
                }

                if is_def_start(&s) && self.contains_target(&s) {
                    target_line = true;
                }
            }
        }
        if ns.len() > 0 {
            println!("Files with nested targets: {}", ns.len());
            println!("Manually comments out the appropriate sections of these files by hand.");
            print_vec(&ns);
            true
        } else {
            println!("No nested targets.");
            false
        }
    }
}

impl<'a> Default for Search<'a> {
    fn default() -> Search<'a> {
        let buffer1 = include_str!("targets.txt");
        let targets = buffer1.lines().filter(|s| !s.starts_with("//")).collect();
        let buffer2 = include_str!("target_usages.txt");
        let target_usages = buffer2.lines().filter(|s| !s.starts_with("//")).collect();
        Search {
            targets: targets,
            target_usages: target_usages,
        }
    }
}

impl<'a> fmt::Display for Search<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s_targets = self.targets
            .iter()
            .filter(|k| **k != "__TESTING_TARGET_DO_NOT_REMOVE__")
            .fold("\n  [ ".to_string(), |mut acc, k| {
                acc.push_str("\n    ");
                acc.push_str(*k);
                acc
            });
        let s_targets = format!("Target identifiers: {}\n  ]", s_targets);

        let s_target_usages = self.target_usages
            .iter()
            .fold("\n  [ ".to_string(), |mut acc, k| {
                acc.push_str("\n    ");
                acc.push_str(*k);
                acc
            });
        let s_target_usages = format!("\nSpecific target usages: {}\n  ]", s_target_usages);

        write!(f, "{}{}", s_targets, s_target_usages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_target_test() {
        let search: Search = Default::default();
        assert!(search.contains_target("__TESTING_TARGET_DO_NOT_REMOVE__"));
    }
}
