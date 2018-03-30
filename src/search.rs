#![allow(dead_code)]
use std::collections::HashSet;
use std::fmt;

use lib::*;

pub struct Search {
    pub targets: HashSet<String>,
    pub affirmative_targets: HashSet<String>,
    pub non_affirmative_targets: HashSet<String>,
}

impl<'a> Search {
    pub fn debug_print_unique_target_ifs(&self, files: &Vec<(String, String)>) {
        let mut xs = vec![];
        let mut ys = vec![];
        for &(ref fully_qualified_file, ref _file) in files {
            let ss = read_file(&fully_qualified_file);
            for s in ss {
                let s_trim = s.trim().to_string();
                if is_start(&s_trim) && self.targets.contains(&s) {
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
