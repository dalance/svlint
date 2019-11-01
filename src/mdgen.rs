#![allow(dead_code)]

mod config;
mod linter;
mod printer;
mod rules;

use crate::config::Config;
use std::fs::File;
use std::io::{BufReader, Read};

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let rules = Config::gen_all_rules();
    println!("# Rules\n");
    for rule in rules {
        println!("## {}\n", rule.name());

        println!("### Description\n");
        println!("{}\n", rule.hint());

        println!("### Reason\n");
        println!("{}\n", rule.reason());

        println!("### Pass example\n");
        let f = File::open(format!("testcases/pass/{}.sv", rule.name())).unwrap();
        let mut f = BufReader::new(f);
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        println!("```SystemVerilog\n{}```\n", s);

        println!("### Fail example\n");
        let f = File::open(format!("testcases/fail/{}.sv", rule.name())).unwrap();
        let mut f = BufReader::new(f);
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        println!("```SystemVerilog\n{}```\n", s);
    }
}
