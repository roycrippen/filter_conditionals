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
