mod config;
mod linter;
mod printer;
mod rules;

use crate::config::Config;
use crate::linter::Linter;
use crate::printer::Printer;
use failure::{Error, Fail, ResultExt};
use regex::Regex;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, process};
use structopt::{clap, StructOpt};
use sv_parser::{parse_sv, ErrorKind};

// -------------------------------------------------------------------------------------------------
// Opt
// -------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "svlint")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    /// Source file
    #[structopt(required_unless_one = &["filelist", "example", "update-config"])]
    pub files: Vec<PathBuf>,

    /// File list
    #[structopt(short = "f", long = "filelist", conflicts_with = "files")]
    pub filelist: Vec<PathBuf>,

    /// Define
    #[structopt(short = "d", long = "define", multiple = true, number_of_values = 1)]
    pub defines: Vec<String>,

    /// Include path
    #[structopt(short = "i", long = "include", multiple = true, number_of_values = 1)]
    pub includes: Vec<PathBuf>,

    /// Config file
    #[structopt(short = "c", long = "config", default_value = ".svlint.toml")]
    pub config: PathBuf,

    /// Prints results by single line
    #[structopt(short = "1")]
    pub single: bool,

    /// Suppresses message
    #[structopt(short = "s", long = "silent")]
    pub silent: bool,

    /// Prints verbose message
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Updates config
    #[structopt(long = "update")]
    pub update_config: bool,

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
            let mut printer = Printer::new();
            let _ = printer.print_error(&format!("{}", x));
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
            .with_context(|_| format!("failed to open '{}'", config.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        let ret = toml::from_str(&s)
            .with_context(|_| format!("failed to parse toml '{}'", config.to_string_lossy()))?;

        if opt.update_config {
            let mut f = OpenOptions::new()
                .write(true)
                .open(&config)
                .with_context(|_| format!("failed to open '{}'", config.to_string_lossy()))?;
            write!(f, "{}", toml::to_string(&ret).unwrap())
                .with_context(|_| format!("failed to write '{}'", config.to_string_lossy()))?;
            return Ok(true);
        }

        ret
    } else {
        println!(
            "Config file '{}' is not found. Use default config",
            opt.config.to_string_lossy()
        );
        Config::new()
    };

    run_opt_config(opt, config)
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt_config(opt: &Opt, config: Config) -> Result<bool, Error> {
    let linter = Linter::new(config);
    let mut printer = Printer::new();

    let mut defines = HashMap::new();
    for define in &opt.defines {
        defines.insert(define.clone(), None);
    }

    let (files, includes) = if !opt.filelist.is_empty() {
        let mut files = Vec::new();
        let mut includes = Vec::new();

        for filelist in &opt.filelist {
            let (mut f, mut i) = parse_filelist(filelist)?;
            files.append(&mut f);
            includes.append(&mut i);
        }

        (files, includes)
    } else {
        (opt.files.clone(), opt.includes.clone())
    };

    let mut all_pass = true;

    for path in &files {
        let mut pass = true;
        match parse_sv(&path, &defines, &includes) {
            Ok((syntax_tree, new_defines)) => {
                for node in &syntax_tree {
                    for failed in linter.check(&syntax_tree, &node) {
                        pass = false;
                        if !opt.silent {
                            printer.print(&failed, opt.single)?;
                        }
                    }
                }
                defines = new_defines;
            }
            Err(x) => {
                print_parse_error(&mut printer, x)?;
                pass = false;
            }
        }

        if pass {
            if opt.verbose {
                printer.print_info(&format!("pass '{}'", path.to_string_lossy()))?;
            }
        } else {
            all_pass = false;
        }
    }

    Ok(all_pass)
}

#[cfg_attr(tarpaulin, skip)]
fn print_parse_error(printer: &mut Printer, error: sv_parser::Error) -> Result<(), Error> {
    match error.kind() {
        ErrorKind::Parse(Some((path, pos))) => {
            printer.print_parse_error(&path, *pos)?;
        }
        ErrorKind::Include => {
            let x = error
                .cause()
                .unwrap()
                .downcast_ref::<sv_parser::Error>()
                .unwrap();
            match x.kind() {
                ErrorKind::File(x) => {
                    printer.print_error(&format!("failed to include '{}'", x.to_string_lossy()))?;
                }
                _ => (),
            }
        }
        ErrorKind::DefineArgNotFound(x) => {
            printer.print_error(&format!("define argument '{}' is not found", x))?;
        }
        ErrorKind::DefineNotFound(x) => {
            printer.print_error(&format!("define '{}' is not found", x))?;
        }
        x => {
            printer.print_error(&format!("{}", x))?;
        }
    }

    Ok(())
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

#[cfg_attr(tarpaulin, skip)]
fn parse_filelist(path: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), Error> {
    let mut f = File::open(path)
        .with_context(|_| format!("failed to open '{}'", path.to_string_lossy()))?;
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let mut files = Vec::new();
    let mut includes = Vec::new();
    let re_env = Regex::new(r"\$\{(?P<env>[^}]+)\}").unwrap();

    for line in s.lines() {
        let line = line.trim();
        if !line.starts_with("//") && line != "" {
            // Expand environment variable
            let mut expanded_line = String::from(line);
            for caps in re_env.captures_iter(&line) {
                let env = &caps["env"];
                if let Ok(env_var) = std::env::var(env) {
                    expanded_line = expanded_line.replace(&format!("${{{}}}", env), &env_var);
                }
            }

            if expanded_line.starts_with("+incdir") {
                for dir in expanded_line.trim_start_matches("+incdir+").split("+") {
                    includes.push(PathBuf::from(dir));
                }
            } else if expanded_line.starts_with("-f ") {
                let path = expanded_line.trim_start_matches("-f ");
                let (mut f, mut i) = parse_filelist(&PathBuf::from(path))?;
                files.append(&mut f);
                includes.append(&mut i);
            } else {
                files.push(PathBuf::from(expanded_line));
            }
        }
    }

    Ok((files, includes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(name: &str, silent: bool) {
        let s = format!("[rules]\n{} = true", name);
        let config: Config = toml::from_str(&s).unwrap();

        let file = format!("testcases/pass/{}.sv", name);
        let args = if silent {
            vec!["svlint", "--silent", &file]
        } else {
            vec!["svlint", &file]
        };
        let opt = Opt::from_iter(args.iter());
        let ret = run_opt_config(&opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let file = format!("testcases/fail/{}.sv", name);
        let args = if silent {
            vec!["svlint", "--silent", &file]
        } else {
            vec!["svlint", &file]
        };
        let opt = Opt::from_iter(args.iter());
        let ret = run_opt_config(&opt, config.clone());
        assert_eq!(ret.unwrap(), false);

        let file = format!("testcases/fail/{}.sv", name);
        let args = if silent {
            vec!["svlint", "-1", "--silent", &file]
        } else {
            vec!["svlint", "-1", &file]
        };
        let opt = Opt::from_iter(args.iter());
        let ret = run_opt_config(&opt, config.clone());
        assert_eq!(ret.unwrap(), false);
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}
