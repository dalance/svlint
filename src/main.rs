use anyhow::{Context, Error};
use clap::Parser;
use enquote;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, process};
use sv_filelist_parser;
use sv_parser::Error as SvParserError;
use sv_parser::{parse_sv, preprocess, Define, DefineText};
use svlint::config::Config;
use svlint::linter::Linter;
use svlint::printer::Printer;

// -------------------------------------------------------------------------------------------------
// Opt
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[clap(name = "svlint")]
#[clap(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
pub struct Opt {
    /// Source file
    #[clap(required_unless_present_any = &["filelist", "example", "update-config"])]
    pub files: Vec<PathBuf>,

    /// File list
    #[clap(short = 'f', long = "filelist", conflicts_with = "files")]
    pub filelist: Vec<PathBuf>,

    /// Define
    #[clap(
        short = 'd',
        long = "define",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub defines: Vec<String>,

    /// Include directory path
    #[clap(
        short = 'i',
        long = "include",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub includes: Vec<PathBuf>,

    /// TOML configuration file
    #[clap(short = 'c', long = "config", default_value = ".svlint.toml")]
    pub config: PathBuf,

    /// Plugin file
    #[clap(
        short = 'p',
        long = "plugin",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub plugins: Vec<PathBuf>,

    /// Ignore any include
    #[clap(long = "ignore-include")]
    pub ignore_include: bool,

    /// Print results by single line
    #[clap(short = '1')]
    pub single: bool,

    /// Suppress messages
    #[clap(short = 's', long = "silent")]
    pub silent: bool,

    /// Print verbose messages
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Print message for GitHub Actions
    #[clap(long = "github-actions")]
    pub github_actions: bool,

    /// Update configuration
    #[clap(long = "update")]
    pub update_config: bool,

    /// Print TOML configuration example
    #[clap(long = "example")]
    pub example: bool,

    /// Print data from filelists
    #[clap(long = "dump-filelist")]
    pub dump_filelist: bool,

    /// Print syntax trees
    #[clap(long = "dump-syntaxtree")]
    pub dump_syntaxtree: bool,

    /// Print preprocessor output instead of performing checks
    #[clap(short = 'E')]
    pub preprocess_only: bool,
}

// -------------------------------------------------------------------------------------------------
// Main
// -------------------------------------------------------------------------------------------------

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let opt = Parser::parse();
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
            let _ = printer.print_error_type(x);
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
            .with_context(|| format!("failed to open '{}'", config.to_string_lossy()))?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        let mut ret: Config = toml::from_str(&s)
            .with_context(|| format!("failed to parse toml '{}'", config.to_string_lossy()))?;

        if opt.update_config {
            ret.migrate();
            let mut f = OpenOptions::new()
                .write(true)
                .open(&config)
                .with_context(|| format!("failed to open '{}'", config.to_string_lossy()))?;
            write!(f, "{}", toml::to_string(&ret).unwrap())
                .with_context(|| format!("failed to write '{}'", config.to_string_lossy()))?;
            return Ok(true);
        }

        ret
    } else {
        if !opt.silent && !opt.dump_filelist && !opt.preprocess_only {
            println!(
                "Config file '{}' is not found. Enable all rules",
                opt.config.to_string_lossy()
            );
        }
        Config::new().enable_all()
    };

    run_opt_config(opt, config)
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt_config(opt: &Opt, config: Config) -> Result<bool, Error> {
    let mut printer = Printer::new();

    let mut not_obsolete = true;
    for (org_rule, renamed_rule) in config.check_rename() {
        printer.print_warning(&format!(
            "Rule \"{}\" is obsolete. Please rename to \"{}\"",
            org_rule, renamed_rule,
        ))?;
        not_obsolete = false;
    }

    let mut linter = Linter::new(config);
    for plugin in &opt.plugins {
        linter.load(&plugin);
    }

    let mut defines = HashMap::new();
    for define in &opt.defines {
        let mut define = define.splitn(2, '=');
        let ident = String::from(define.next().unwrap());
        let text = if let Some(x) = define.next() {
            let x = enquote::unescape(x, None)?;
            Some(DefineText::new(x, None))
        } else {
            None
        };
        let define = Define::new(ident.clone(), vec![], text);
        defines.insert(ident, Some(define));
    }

    let (files, includes) = if !opt.filelist.is_empty() {
        let mut files = opt.files.clone();
        let mut includes = opt.includes.clone();

        for filelist in &opt.filelist {
            let (mut f, mut i, d) = parse_filelist(filelist)?;
            if opt.dump_filelist {
                dump_filelist(&mut printer, &filelist, &f, &i, &d);
            }
            files.append(&mut f);
            includes.append(&mut i);
            for (k, v) in d {
                defines.insert(k, v);
            }
        }

        (files, includes)
    } else {
        (opt.files.clone(), opt.includes.clone())
    };

    if opt.dump_filelist {
        dump_filelist(&mut printer, &Path::new("."), &files, &includes, &defines);
        return Ok(true);
    }

    let mut all_pass = true;

    for path in &files {
        let mut pass = true;
        if opt.preprocess_only {
            match preprocess(&path, &defines, &includes, false, opt.ignore_include) {
                Ok((text, new_defines)) => {
                    print!("{}", text.text());
                    defines = new_defines;
                }
                Err(x) => {
                    print_parser_error(&mut printer, x, opt.single)?;
                    pass = false;
                }
            }
        } else {
            match parse_sv(&path, &defines, &includes, opt.ignore_include, false) {
                Ok((syntax_tree, new_defines)) => {
                    for node in syntax_tree.into_iter().event() {
                        for failed in linter.check(&syntax_tree, &node) {
                            pass = false;
                            if !opt.silent {
                                printer.print_failed(&failed, opt.single, opt.github_actions)?;
                            }
                        }
                    }
                    defines = new_defines;

                    if opt.dump_syntaxtree {
                        println!("{:?}", &syntax_tree);
                    }
                }
                Err(x) => {
                    print_parser_error(&mut printer, x, opt.single)?;
                    pass = false;
                }
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

    Ok(all_pass && not_obsolete)
}

#[cfg_attr(tarpaulin, skip)]
fn print_parser_error(
    printer: &mut Printer,
    error: SvParserError,
    single: bool,
) -> Result<(), Error> {
    match error {
        SvParserError::Parse(Some((path, pos))) => {
            printer.print_parse_error(&path, pos, single)?;
        }
        SvParserError::Preprocess(Some((path, pos))) => {
            printer.print_preprocess_error(&path, pos, single)?;
        }
        SvParserError::Include { source: x } => {
            if let SvParserError::File { path: x, .. } = *x {
                printer.print_error(&format!("failed to include '{}'", x.to_string_lossy()))?;
            }
        }
        SvParserError::DefineArgNotFound(x) => {
            printer.print_error(&format!("define argument '{}' is not found", x))?;
        }
        SvParserError::DefineNotFound(x) => {
            printer.print_error(&format!("define '{}' is not found", x))?;
        }
        x => {
            printer.print_error(&format!("{}", x))?;
        }
    }

    Ok(())
}

#[cfg_attr(tarpaulin, skip)]
fn search_config(config: &Path) -> Option<PathBuf> {
    if let Ok(c) = env::var("SVLINT_CONFIG") {
        let candidate = Path::new(&c);
        if candidate.exists() {
            return Some(candidate.to_path_buf());
        } else {
            let mut printer = Printer::new();
            printer
                .print_warning(&format!(
                    "SVLINT_CONFIG=\"{}\" does not exist. Searching hierarchically.",
                    c,
                ))
                .ok()?;
        }
    }

    if let Ok(current) = env::current_dir() {
        for dir in current.ancestors() {
            let candidate = dir.join(config);
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
fn parse_filelist(
    path: &Path,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>, HashMap<String, Option<Define>>), Error> {
    let filelist = match sv_filelist_parser::parse_file(path) {
        Ok(f) => f,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "failed to open '{}'",
                path.to_string_lossy()
            ))
        }
    };
    let mut defines = HashMap::new();
    for (d, t) in filelist.defines {
        match t {
            Some(t) => {
                let define_text = DefineText::new(String::from(&t), None);
                let define = Define::new(String::from(&d), vec![], Some(define_text));
                defines.insert(String::from(&d), Some(define));
            }
            None => {
                defines.insert(String::from(&d), None);
            }
        }
    }

    Ok((filelist.files, filelist.incdirs, defines))
}

fn dump_filelist(
    printer: &mut Printer,
    filename: &Path,
    files: &Vec<PathBuf>,
    incdirs: &Vec<PathBuf>,
    defines: &HashMap<String, Option<Define>>,
) -> () {
    printer.println(format!("{:?}:", filename).as_str());

    printer.println(format!("  files:").as_str());
    for f in files {
        printer.println(format!("    - {:?}", f).as_str());
    }

    printer.println(format!("  incdirs:").as_str());
    for i in incdirs {
        printer.println(format!("    - {:?}", i).as_str());
    }

    printer.println(format!("  defines:").as_str());
    for (k, v) in defines {
        match v {
            None => printer.println(format!("    {:?}:", k).as_str()),
            Some(define) => match &define.text {
                Some(definetext) => printer.println(format!("    {:?}: {:?}", k, definetext.text).as_str()),
                None => printer.println(format!("    {:?}:", k).as_str()),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(rulename: &str, filename: &str, pass_not_fail: bool, silent: bool, oneline: bool) {
        let s = format!("[rules]\n{} = true", rulename);
        let config: Config = toml::from_str(&s).unwrap();

        let mut args = vec!["svlint"];
        if silent {
            args.push("--silent");
        }
        if oneline {
            args.push("-1");
        }
        args.push(filename);
        let opt = Opt::parse_from(args.iter());

        let ret = run_opt_config(&opt, config.clone());
        assert_eq!(ret.unwrap(), pass_not_fail);
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));

    // TODO: Tests for --dump-filelist
}
