#![allow(dead_code)]

mod config;
mod linter;
mod printer;
mod rules;

use crate::config::{Config, ConfigOption};
use crate::linter::Rule;
use regex::Regex;
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

fn partition_rules(
    rules: Vec<Box<dyn Rule>>,
) -> (Vec<Box<dyn Rule>>, Vec<Box<dyn Rule>>, Vec<Box<dyn Rule>>) {
    let style_prefixes = ["style_", "tab_"].join("|");
    let re_style: Regex = Regex::new(format!("^({})", style_prefixes).as_str()).unwrap();

    let naming_prefixes = ["prefix_", "lowercamelcase_", "uppercamelcase_", "re_"].join("|");
    let re_naming: Regex =
        Regex::new(format!("(^({})|_with_label$)", naming_prefixes).as_str()).unwrap();

    let mut ruleset_style: Vec<Box<dyn Rule>> = Vec::new();
    let mut ruleset_naming: Vec<Box<dyn Rule>> = Vec::new();
    let mut ruleset_functional: Vec<Box<dyn Rule>> = Vec::new();

    for rule in rules {
        if re_style.is_match(&rule.name()) {
            ruleset_style.push(rule);
        } else if re_naming.is_match(&rule.name()) {
            ruleset_naming.push(rule);
        } else {
            ruleset_functional.push(rule);
        }
    }

    (ruleset_functional, ruleset_naming, ruleset_style)
}

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let (functional_rules, naming_rules, style_rules) = partition_rules(Config::gen_all_rules());

    println!(
        "{}\n",
        file_contents(format!("md/manual-introduction.md").as_str())
    );

    println!(
        "{}\n",
        file_contents(format!("md/manual-functional_rules.md").as_str())
    );
    print_rules(functional_rules);

    println!(
        "{}\n",
        file_contents(format!("md/manual-naming_convention_rules.md").as_str())
    );
    print_rules(naming_rules);

    println!(
        "{}\n",
        file_contents(format!("md/manual-style_convention_rules.md").as_str())
    );
    print_rules(style_rules);
}
