#![allow(dead_code)]
use std::collections::HashSet;
use std::fmt;

use lib::*;

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
                let s_trim = s.trim().to_string();
                if is_start(&s_trim) && self.contains_target(&s) {
                    if is_affirmative(&s) {
                        xs.push(s_trim);
                    } else {
                        ys.push(s_trim);
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
        println!("")
    }

    // find nested affirmative targets
    pub fn get_nested_affirmative_target(&self, ss: &Vec<String>, file: &String) -> Nested {
        let mut target_line = false;
        let mut nested: Nested = Default::default();
        for (i, s) in ss.into_iter().enumerate() {
            let s = s.trim();
            if target_line {
                if is_start(s) || s.starts_with("#ifndef") {
                    nested.conditional = s.clone().to_string();
                    nested.start_line = i + 1;
                    nested.file = file.clone();
                }

                if s.starts_with("#endif") {
                    if nested.start_line > 0 {
                        nested.end_line = i + 1;
                        return nested;
                    }
                    target_line = false;
                }
            }

            if is_start(s) && is_affirmative(s) && self.contains_target_usage(s) {
                target_line = true;
            }
        }
        nested
    }
}

impl<'a> Default for Search<'a> {
    fn default() -> Search<'a> {
        let buffer1 = include_str!("targets.txt");
        let targets = buffer1.lines().filter(|s| !s.starts_with("//")).collect();
        let buffer2 = include_str!("target_affirms.txt");
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
            .fold("\n  [ ".to_string(), |mut acc, k| {
                acc.push_str("\n    ");
                acc.push_str(*k);
                acc
            });
        let s_targets = format!("Target identifiers: {}\n  ]\n", s_targets);

        let s_target_usages = self.target_usages
            .iter()
            .fold("\n  [ ".to_string(), |mut acc, k| {
                acc.push_str("\n    ");
                acc.push_str(*k);
                acc
            });
        let s_target_usages = format!("\nSpecific target usages: {}\n  ]\n", s_target_usages);

        write!(f, "{}{}", s_targets, s_target_usages)
    }
}
