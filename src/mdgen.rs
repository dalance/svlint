#![allow(dead_code)]

mod config;
mod linter;
mod printer;
mod rules;

use crate::config::{Config, ConfigOption};
use crate::linter::Rule;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

fn file_contents(path: &str) -> String {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&cargo_manifest_dir).join(path);

    let file: File = File::open(path).unwrap();
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents
}

fn write_md_rules(o: &mut File, rules: Vec<Box<dyn Rule>>) -> () {
    for rule in rules {
        let _ = writeln!(o, "---");
        let _ = writeln!(o, "## `{}`\n", rule.name());

        let _ = writeln!(o, "### Hint\n");
        let _ = writeln!(o, "{}\n", rule.hint(&ConfigOption::default()));

        let _ = writeln!(o, "### Reason\n");
        let _ = writeln!(o, "{}\n", rule.reason());

        let _ = writeln!(o, "### Pass Example\n");
        let p: String = format!("testcases/pass/{}.sv", rule.name());
        let _ = writeln!(o, "```SystemVerilog\n{}```\n", file_contents(&p));

        let _ = writeln!(o, "### Fail Example\n");
        let p: String = format!("testcases/fail/{}.sv", rule.name());
        let _ = writeln!(o, "```SystemVerilog\n{}```\n", file_contents(&p));

        let _ = writeln!(o, "### Explanation\n");
        let p: String = format!("md/explanation-{}.md", rule.name());
        let _ = writeln!(o, "{}\n", file_contents(&p));
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

fn write_manual_md() -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let o = Path::new(&cargo_manifest_dir).join("RULES.md");
    let mut o = File::create(&o).unwrap();

    let (functional_rules, naming_rules, style_rules) = partition_rules(Config::gen_all_rules());

    let _ = writeln!(
        o,
        "{}\n",
        file_contents(format!("md/manual-introduction.md").as_str())
    );

    let _ = writeln!(
        o,
        "{}\n",
        file_contents(format!("md/manual-functional_rules.md").as_str())
    );
    write_md_rules(&mut o, functional_rules);

    let _ = writeln!(
        o,
        "{}\n",
        file_contents(format!("md/manual-naming_convention_rules.md").as_str())
    );
    write_md_rules(&mut o, naming_rules);

    let _ = writeln!(
        o,
        "{}\n",
        file_contents(format!("md/manual-style_convention_rules.md").as_str())
    );
    write_md_rules(&mut o, style_rules);
}

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    write_manual_md();
}
