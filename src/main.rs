mod config;
mod linter;
mod printer;
mod rules;

use crate::config::Config;
use crate::linter::Linter;
use crate::printer::Printer;
use failure::{Error, ResultExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, process};
use structopt::{clap, StructOpt};
use sv_parser::parse_sv;

// -------------------------------------------------------------------------------------------------
// Opt
// -------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "svlint")]
#[structopt(long_version("option_env!(\"LONG_VERSION\").unwrap_or(env!(\"CARGO_PKG_VERSION\"))"))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    /// Source file
    pub files: Vec<PathBuf>,

    /// Define
    #[structopt(short = "d", long = "define", multiple = true, number_of_values = 1)]
    pub defines: Vec<String>,

    /// Include path
    #[structopt(short = "i", long = "include", multiple = true, number_of_values = 1)]
    pub includes: Vec<PathBuf>,

    /// Config file
    #[structopt(short = "c", long = "config", default_value = ".svlint.toml")]
    pub config: PathBuf,

    /// Show results by simple format
    #[structopt(short = "s", long = "simple")]
    pub simple: bool,

    /// Show verbose message
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Prints config example
    #[structopt(long = "example")]
    pub example: bool,
}

// -------------------------------------------------------------------------------------------------
// Main
// -------------------------------------------------------------------------------------------------

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let opt = Opt::from_args();
    let exit_code = match run_opt(&opt) {
        Ok(pass) => {
            if pass {
                0
            } else {
                1
            }
        }
        Err(x) => {
            println!("Error: {}", x);
            2
        }
    };

    process::exit(exit_code);
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt(opt: &Opt) -> Result<bool, Error> {
    if opt.example {
        let config = Config::new();
        println!("{}", toml::to_string(&config).unwrap());
        return Ok(true);
    }

    let config = search_config(&opt.config);

    let config = if let Some(config) = config {
        let mut f = File::open(&config)
            .with_context(|_| format!("failed to open: '{}'", config.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        toml::from_str(&s)
            .with_context(|_| format!("failed to parse toml: '{}'", config.to_string_lossy()))?
    } else {
        Config::new()
    };

    let linter = Linter::new(config);
    let mut printer = Printer::new();

    let mut defines = HashMap::new();
    for define in &opt.defines {
        defines.insert(define.clone(), None);
    }

    let mut pass = true;

    for path in &opt.files {
        match parse_sv(&path, &defines, &opt.includes) {
            Ok((syntax_tree, new_defines)) => {
                for node in &syntax_tree {
                    for failed in linter.check(&syntax_tree, &node) {
                        pass = false;
                        printer.print(&failed, opt.simple)?;
                    }
                }
                defines = new_defines;
            }
            Err(_) => {}
        }
    }

    Ok(pass)
}

#[cfg_attr(tarpaulin, skip)]
fn search_config(rule: &Path) -> Option<PathBuf> {
    if let Ok(current) = env::current_dir() {
        for dir in current.ancestors() {
            let candidate = dir.join(rule);
            if candidate.exists() {
                return Some(candidate);
            }
        }
        None
    } else {
        None
    }
}
