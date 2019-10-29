mod config;
mod linter;
mod printer;
mod rules;

use crate::config::Config;
use crate::linter::Linter;
use crate::printer::Printer;
use failure::{Error, Fail, ResultExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, process};
use structopt::{clap, StructOpt};
use sv_parser::{parse_sv, ErrorKind};

// -------------------------------------------------------------------------------------------------
// Opt
// -------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "svlint")]
#[structopt(long_version("option_env!(\"LONG_VERSION\").unwrap_or(env!(\"CARGO_PKG_VERSION\"))"))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    /// Source file
    #[structopt(required_unless = "filelist")]
    pub files: Vec<PathBuf>,

    /// File list
    #[structopt(short = "f", long = "filelist", conflicts_with = "files")]
    pub filelist: Option<PathBuf>,

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
        toml::from_str(&s)
            .with_context(|_| format!("failed to parse toml '{}'", config.to_string_lossy()))?
    } else {
        Config::new()
    };

    let linter = Linter::new(config);
    let mut printer = Printer::new();

    let mut defines = HashMap::new();
    for define in &opt.defines {
        defines.insert(define.clone(), None);
    }

    let files = if let Some(ref filelist) = opt.filelist {
        let mut f = File::open(&filelist)
            .with_context(|_| format!("failed to open '{}'", filelist.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        let mut files = Vec::new();

        for line in s.lines() {
            if !line.starts_with("//") {
                files.push(PathBuf::from(line));
            }
        }

        files
    } else {
        opt.files.clone()
    };

    let mut all_pass = true;

    for path in &files {
        let mut pass = true;
        match parse_sv(&path, &defines, &opt.includes) {
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
        ErrorKind::DefineTextNotFound(x) => {
            printer.print_error(&format!("define text of '{}' is not found", x))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test(name: &str, silent: bool) {
        let file = format!("testcases/pass/{}.sv", name);
        let args = if silent {
            vec!["svlint", "--silent", &file]
        } else {
            vec!["svlint", &file]
        };
        let opt = Opt::from_iter(args.iter());
        let ret = run_opt(&opt);
        assert_eq!(ret.unwrap(), true);

        let file = format!("testcases/fail/{}.sv", name);
        let args = if silent {
            vec!["svlint", "--silent", &file]
        } else {
            vec!["svlint", &file]
        };
        let opt = Opt::from_iter(args.iter());
        let ret = run_opt(&opt);
        assert_eq!(ret.unwrap(), false);
    }

    #[test]
    fn test_enum_with_type() {
        test("enum_with_type", true);
    }

    #[test]
    fn test_function_with_automatic() {
        test("function_with_automatic", true);
    }

    #[test]
    fn test_generate_for_with_label() {
        test("generate_for_with_label", true);
    }

    #[test]
    fn test_generate_if_with_label() {
        test("generate_if_with_label", true);
    }

    #[test]
    fn test_generate_keyword() {
        test("generate_keyword", true);
    }

    #[test]
    fn test_genvar_declaration() {
        test("genvar_declaration", true);
    }

    #[test]
    fn test_if_with_begin() {
        test("if_with_begin", true);
    }

    #[test]
    fn test_inout_with_tri() {
        test("inout_with_tri", true);
    }

    #[test]
    fn test_input_with_var() {
        test("input_with_var", true);
    }

    #[test]
    fn test_legacy_always() {
        test("legacy_always", true);
    }

    #[test]
    fn test_loop_variable_declaration() {
        test("loop_variable_declaration", true);
    }

    #[test]
    fn test_output_with_var() {
        test("output_with_var", true);
    }

    #[test]
    fn test_parameter_in_package() {
        test("parameter_in_package", true);
    }

    #[test]
    fn test_priority_keyword() {
        test("priority_keyword", true);
    }

    #[test]
    fn test_tab_charactor() {
        test("tab_charactor", true);
    }

    #[test]
    fn test_unique0_keyword() {
        test("unique0_keyword", true);
    }

    #[test]
    fn test_unique_keyword() {
        test("unique_keyword", true);
    }

    #[test]
    fn test_wire_reg() {
        test("wire_reg", true);
    }
}
