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
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DumpFilelistMode {
    No,
    Yaml,
    Files,
    Incdirs,
    Defines,
}

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
    #[clap(value_enum, default_value = "no", long = "dump-filelist")]
    pub dump_filelist: DumpFilelistMode,

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
    let mut printer = Printer::new(false);
    let exit_code = match run_opt(&mut printer, &opt) {
        Ok(pass) => {
            if pass {
                0
            } else {
                1
            }
        }
        Err(x) => {
            let _ = printer.print_error_type(x);
            2
        }
    };

    process::exit(exit_code);
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt(printer: &mut Printer, opt: &Opt) -> Result<bool, Error> {
    if opt.example {
        let config = Config::new();
        let config = format!("{}", toml::to_string(&config).unwrap());
        printer.println(&config)?;
        return Ok(true);
    }

    let config = search_config(printer, &opt.config);

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
        let do_dump_filelist: bool = match opt.dump_filelist {
            DumpFilelistMode::No => false,
            _ => true,
        };

        if !opt.silent && !do_dump_filelist && !opt.preprocess_only {
            let msg = format!(
                "Config file '{}' is not found. Enable all rules",
                opt.config.to_string_lossy()
            );
            printer.print_warning(&msg)?;
        }
        Config::new().enable_all()
    };

    run_opt_config(printer, opt, config)
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt_config(printer: &mut Printer, opt: &Opt, config: Config) -> Result<bool, Error> {
    let mut not_obsolete = true;
    for (org_rule, renamed_rule) in config.check_rename() {
        let msg = format!(
            "Rule \"{}\" is obsolete. Please rename to \"{}\"",
            org_rule, renamed_rule,
        );
        printer.print_warning(&msg)?;
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
            if let DumpFilelistMode::Yaml = opt.dump_filelist {
                dump_filelist(printer, &opt.dump_filelist, &filelist, &f, &i, &d)?;
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

    match opt.dump_filelist {
        DumpFilelistMode::No => {}
        _ => {
            dump_filelist(printer, &opt.dump_filelist, &Path::new("."), &files, &includes, &defines)?;
            return Ok(true);
        }
    }

    let mut all_pass = true;

    for path in &files {
        let mut pass = true;
        if opt.preprocess_only {
            match preprocess(&path, &defines, &includes, false, opt.ignore_include) {
                Ok((text, new_defines)) => {
                    let msg = format!("{}", text.text());
                    printer.print(&msg)?;
                    defines = new_defines;
                }
                Err(x) => {
                    print_parser_error(printer, x, opt.single)?;
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
                        let msg = format!("{:?}", &syntax_tree);
                        printer.println(&msg)?;
                    }
                }
                Err(x) => {
                    print_parser_error(printer, x, opt.single)?;
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
        SvParserError::ReadUtf8(path) => {
            printer.print_error(&format!("file '{}' is not valid UTF-8", path.display()))?;
        }
        SvParserError::DefineNoArgs(x) => {
            printer.print_error(&format!("macro '{}' requires arguments", x))?;
        }
        SvParserError::DefineArgNotFound(x) => {
            printer.print_error(&format!("macro argument '{}' is required", x))?;
        }
        SvParserError::DefineNotFound(x) => {
            printer.print_error(&format!("macro '{}' is not defined", x))?;
        }
        x => {
            printer.print_error(&format!("{}", x))?;
        }
    }

    Ok(())
}

#[cfg_attr(tarpaulin, skip)]
fn search_config(printer: &mut Printer, config: &Path) -> Option<PathBuf> {
    if let Ok(c) = env::var("SVLINT_CONFIG") {
        let candidate = Path::new(&c);
        if candidate.exists() {
            return Some(candidate.to_path_buf());
        } else {
            let msg = format!(
                "SVLINT_CONFIG=\"{}\" does not exist. Searching hierarchically.",
                c,
            );
            printer.print_warning(&msg).ok()?;
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
    mode: &DumpFilelistMode,
    filename: &Path,
    files: &Vec<PathBuf>,
    incdirs: &Vec<PathBuf>,
    defines: &HashMap<String, Option<Define>>,
) -> Result<(), Error> {
    match mode {
        DumpFilelistMode::Yaml => {
            printer.println(&format!("\"{}\":", filename.display()))?;

            printer.println(&format!("  files:"))?;
            for f in files {
                printer.println(&format!("    - \"{}\"", f.display()))?;
            }

            printer.println(&format!("  incdirs:"))?;
            for i in incdirs {
                printer.println(&format!("    - \"{}\"", i.display()))?;
            }

            printer.println(&format!("  defines:"))?;
            let mut keys: Vec<&String> = defines.keys().collect();
            keys.sort_unstable();
            for k in keys {
                let v = defines.get(k).unwrap();

                match v {
                    None => printer.println(&format!("    \"{}\":", k)),
                    Some(define) => match &define.text {
                        Some(definetext) => printer.println(&format!("    \"{}\": \"{}\"", k, definetext.text)),
                        None => printer.println(&format!("    \"{}\":", k)),
                    },
                }?;
            }
        }
        DumpFilelistMode::Files => {
            for f in files {
                printer.println(&format!("{}", f.display()))?;
            }
        }
        DumpFilelistMode::Incdirs => {
            for i in incdirs {
                printer.println(&format!("{}", i.display()))?;
            }
        }
        DumpFilelistMode::Defines => {
            let mut keys: Vec<&String> = defines.keys().collect();
            keys.sort_unstable();
            for k in keys {
                let v = defines.get(k).unwrap();

                match v {
                    None => printer.println(&format!("{}", k)),
                    Some(define) => match &define.text {
                        Some(definetext) => printer.println(&format!("{}={}", k, definetext.text)),
                        None => printer.println(&format!("{}=", k)),
                    },
                }?;
            }
        }
        _ => {}
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::io::BufReader;

    fn resources_path(s: &str) -> String {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(cargo_manifest_dir.as_str())
            .join("testcases")
            .join("resources")
            .join(s);

        String::from(path.to_str().unwrap())
    }

    // Take contents from a file in `testcases/expected/` and convert them to
    // the platform-specific format given on stdout.
    // On Unix, return the files contents (mostly) unchanged.
    // On Windows, change runtime paths, i.e. anything beginning with
    // `$CARGO_MANIFEST_DIR`, to the Unix equivalent and replace Windows line
    // endings (CRLF) with Unix line endings (LF).
    fn expected_contents(s: &str) -> String {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(cargo_manifest_dir.as_str())
            .join("testcases")
            .join("expected")
            .join(s);

        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        if cfg!(windows) {
            // 1. CRLF -> LF
            let ret = contents.replace("\r\n", "\n");

            // 2. "$CARGO_MANIFEST_DIR/foo/bar.baz" -> "$CARGO_MANIFEST_DIR\\foo\\bar.baz"
            let expected_paths = Regex::new(r"\$CARGO_MANIFEST_DIR[a-zA-Z0-9_/]+").unwrap();
            let mut r = ret.clone();
            for cap in expected_paths.captures_iter(&ret) {
                let expected_path = &cap[0];
                let runtime_path = expected_path.replace("/", "\\\\");
                r = r.replace(expected_path, &runtime_path);
            }
            let ret: String = r;

            // 3. "$CARGO_MANIFEST_DIR\\foo\\bar.baz" -> "C:\\path\\svlint\\foo\\bar.baz"
            let cargo_manifest_dir: String = cargo_manifest_dir.escape_default().to_string();
            let ret = ret.replace("$CARGO_MANIFEST_DIR", cargo_manifest_dir.as_str());

            ret
        } else {
            contents.replace("$CARGO_MANIFEST_DIR", cargo_manifest_dir.as_str())
        }
    }

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

        let mut printer = Printer::new(false);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), pass_not_fail);
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));

    #[test]
    fn dump_filelist_1() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Files, not filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("foo/bar/one.sv");
        args.push("foo/bar/two.sv");
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_1"));
    } // }}}

    #[test]
    fn dump_filelist_2() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Single flat filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("--filelist");
        let f_1 = resources_path("child1.fl");
        args.push(&f_1);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_2"));
    } // }}}

    #[test]
    fn dump_filelist_3() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Single non-flat filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("--filelist");
        let f_1 = resources_path("parent1.fl");
        args.push(&f_1);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_3"));
    } // }}}

    #[test]
    fn dump_filelist_4() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Muliple filelists.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("--filelist");
        let f_1 = resources_path("child1.fl");
        args.push(&f_1);
        args.push("--filelist");
        let f_2 = resources_path("child2.fl");
        args.push(&f_2);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_4"));
    } // }}}

    #[test]
    fn dump_filelist_5() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Single deeper filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("--filelist");
        let f_1 = resources_path("grandparent1.fl");
        args.push(&f_1);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_5"));
    } // }}}
}
