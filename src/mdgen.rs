#![allow(dead_code)]

mod config;
mod linter;
mod printer;
mod rules;

use crate::config::{Config, ConfigOption};
use std::fs::File;
use std::io::{BufReader, Read};

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let rules = Config::gen_all_rules();

    println!("# Rules\n");
    println!("This document is generated from the rules' source code (`svlint/src/rules/*.rs`)");
    println!("and testcases (`testcases/(fail|pass)/*.sv`) using the `mdgen` binary.");
    println!("Each rule is documented with 5 pieces of information:");
    println!("- Hint: A brief instruction on how to modify failing SystemVerilog.");
    println!("  Also displayed in supported editors using [svls](https://github.com/dalance/svls).");
    println!("- Reason: A one sentence explanation of the rule's purpose.");
    println!("  Also displayed in supported editors using [svls](https://github.com/dalance/svls).");
    println!("- Pass Example: A valid piece of SystemVerilog which is known to pass the rule.");
    println!("  Ideally, this will show an example of best-practice.");
    println!("- Fail Example: A valid piece of SystemVerilog which is known to fail the rule.");
    println!("  In some cases the code shows multiple commented examples.");
    println!("- Explanation: A full explanation of the rule's purpose with references to any");
    println!("  other relevant information sources.");
    println!("\n");

    for rule in rules {
        println!("---");
        println!("## `{}`\n", rule.name());

        println!("### Hint\n");
        println!("{}\n", rule.hint(&ConfigOption::default()));

        println!("### Reason\n");
        println!("{}\n", rule.reason());

        println!("### Pass Example\n");
        let f = File::open(format!("testcases/pass/{}.sv", rule.name())).unwrap();
        let mut f = BufReader::new(f);
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        println!("```SystemVerilog\n{}```\n", s);

        println!("### Fail Example\n");
        let f = File::open(format!("testcases/fail/{}.sv", rule.name())).unwrap();
        let mut f = BufReader::new(f);
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        println!("```SystemVerilog\n{}```\n", s);

        println!("### Explanation\n");
        println!("{}\n", rule.explanation());
    }
}
