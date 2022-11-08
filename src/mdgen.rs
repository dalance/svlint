#![allow(dead_code)]

mod config;
mod linter;
mod printer;
mod rules;

use crate::config::{Config, ConfigOption};
use crate::linter::Rule;
use std::fs::File;
use std::io::{BufReader, Read};

fn file_contents(path: &str) -> String {
    let file: File = File::open(path).unwrap();
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents
}

fn print_rules(rules: Vec<Box<dyn Rule>>) -> () {
    for rule in rules {
        println!("---");
        println!("## `{}`\n", rule.name());

        println!("### Hint\n");
        println!("{}\n", rule.hint(&ConfigOption::default()));

        println!("### Reason\n");
        println!("{}\n", rule.reason());

        println!("### Pass Example\n");
        let p: String = format!("testcases/pass/{}.sv", rule.name());
        println!("```SystemVerilog\n{}```\n", file_contents(&p));

        println!("### Fail Example\n");
        let p: String = format!("testcases/fail/{}.sv", rule.name());
        println!("```SystemVerilog\n{}```\n", file_contents(&p));

        println!("### Explanation\n");
        let p: String = format!("md/explanation-{}.md", rule.name());
        println!("{}\n", file_contents(&p));
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let rules: Vec<Box<dyn Rule>> = Config::gen_all_rules();

    let p: String = format!("md/manual-introduction.md");
    println!("{}\n", file_contents(&p));

    let p: String = format!("md/manual-rules.md");
    println!("{}\n", file_contents(&p));
    print_rules(rules);
}
