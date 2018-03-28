#![allow(dead_code)]
use std::collections::HashSet;
use std::fmt;

use lib::*;
use metrics::Metrics;

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
    pub fn get_nested_affirmative_target(&self, ss: &Vec<String>) -> Mark {
        let mut target_line = false;
        let mut mark: Mark = Default::default();
        for (i, s) in ss.into_iter().enumerate() {
            let s = s.trim();
            if target_line {
                if is_start(s) || s.starts_with("#ifndef") {
                    mark.conditional = s.clone().to_string();
                    mark.start_line = i + 1;
                }

                if s.starts_with("#endif") {
                    if mark.start_line > 0 {
                        mark.end_line = i + 1;
                        return mark;
                    }
                    target_line = false;
                }
            }

            if is_start(s) && is_affirmative(s) && self.contains_target_usage(s)
                && s.starts_with("#else")
            {
                target_line = true;
            }
        }
        mark
    }

    pub fn process_nested_affirmatives(
        &self,
        ss: &mut Vec<String>,
        file: &str,
        detail_print: bool,
    ) {
        for _i in 0..10 {
            let mut mark = self.get_nested_affirmative_target(&ss);
            if mark.start_line > 0 {
                mark.conditional = "nested conditional".to_string();
                mark.file = file.to_string();
                comment_lines(ss, &mark);
                print_commented_lines(&ss, &mark, detail_print);
            } else {
                break;
            }
        }
    }

    pub fn process_affirmative(
        &self,
        ss: &mut Vec<String>,
        metrics: &mut Metrics,
        file: &str,
        detail_print: bool,
    ) {
        const MARKER: &'static str =
    "=========================================================================================\n";
        for _i in 0..10 {
            let mut target_line = false;
            let mut conditional = "".to_string();
            let mut mark: Mark = Default::default();
            for (i, s) in ss.clone().into_iter().enumerate() {
                let s = s.trim();
                if is_start(s) {
                    if is_affirmative(s) && self.contains_target_usage(s) {
                        conditional = s.to_string();
                        mark.start_line = i + 1;
                        target_line = true;
                        metrics.affected_files.insert(file.to_string().clone());
                        metrics.removed_blocks += 1;
                        metrics.removed_lines.push(file.to_string().clone());
                    }
                }

                if target_line {
                    metrics.removed_lines.push(s.clone().to_string());
                    if is_end(s) {
                        metrics.removed_lines.push(MARKER.to_string());
                        mark.end_line = i + 1;
                        break;
                    }
                }
            }
            if mark.start_line > 0 {
                mark.conditional = conditional.to_string();
                mark.file = file.to_string();
                comment_lines(ss, &mark);
                print_commented_lines(&ss, &mark, detail_print);
            } else {
                break;
            }
        }
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
