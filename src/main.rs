use anyhow::{Context, Error};
use clap::{Parser, CommandFactory};
use clap_complete;
use enquote;
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, process};
use sv_filelist_parser;
use sv_parser::Error as SvParserError;
use sv_parser::{parse_sv_str, preprocess, Define, DefineText};
use svlint::config::Config;
use svlint::linter::Linter;
use svlint::printer::Printer;

// -------------------------------------------------------------------------------------------------
// Opt
// -------------------------------------------------------------------------------------------------
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DumpFilelistMode {
    Yaml,
    Files,
    Incdirs,
    Defines,
}

#[derive(Debug, Parser)]
#[clap(name = "svlint")]
#[clap(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
pub struct Opt {
    /// Source file(s)
    #[clap(required_unless_present_any = &["filelist", "config-example", "config-update", "dump-completion"])]
    pub files: Vec<PathBuf>,

    /// Filelist file(s)
    #[clap(short = 'f', long = "filelist", conflicts_with = "files")]
    pub filelist: Vec<PathBuf>,

    /// Define macro for preprocessor, e.g. `-D FOO` or `-D FOO=123`
    #[clap(
        short = 'D',
        short_alias = 'd',
        long = "define",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub defines: Vec<String>,

    /// Include directory for preprocessor, e.g. `-I path/to/headerfiles/`
    #[clap(
        short = 'I',
        short_alias = 'i',
        long = "incdir",
        alias = "include",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub incdirs: Vec<PathBuf>,

    /// TOML configuration file, searched for hierarchically upwards
    #[clap(short = 'c', long = "config", default_value = ".svlint.toml")]
    pub config: PathBuf,

    /// Plugin file, e.g. `-p path/to/libfoo.so` or `-p path\to\foo.dll`
    #[clap(
        short = 'p',
        long = "plugin",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub plugins: Vec<PathBuf>,

    /// Ignore all preprocessor `include directives
    #[clap(long = "ignore-include")]
    pub ignore_include: bool,

    /// Print one rule failure message per line
    #[clap(short = '1', long = "oneline")]
    pub oneline: bool,

    /// Suppress printing, useful for scripting
    #[clap(short = 's', long = "silent")]
    pub silent: bool,

    /// Print verbose messages
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Format rule failure messages for GitHub Actions
    #[clap(long = "github-actions")]
    pub github_actions: bool,

    /// Update TOML configuration file in-place
    #[clap(long = "config-update", alias = "update")]
    pub config_update: bool,

    /// Print an example TOML configuration
    #[clap(long = "config-example", alias = "example")]
    pub config_example: bool,

    /// Print data from filelists
    #[clap(value_enum, long = "dump-filelist")]
    pub dump_filelist: Option<DumpFilelistMode>,

    /// Print shell completion script
    #[clap(value_enum, long = "dump-completion")]
    pub dump_completion: Option<clap_complete::Shell>,

    /// Print syntax trees, useful for debug or syntax analysis
    #[clap(long = "dump-syntaxtree")]
    pub dump_syntaxtree: bool,

    /// Print preprocessor output then exit before parsing syntax
    #[clap(short = 'E', long = "preprocess-only")]
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
    if opt.config_example {
        let config = Config::new();
        let config = format!("{}", toml::to_string(&config).unwrap());
        printer.println(&config)?;
        return Ok(true);
    }

    if let Some(generator) = opt.dump_completion {
        let mut cmd = Opt::command();
        dump_completion(generator, &mut cmd);
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

        if opt.config_update {
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
        if !opt.plugins.is_empty() {
            Config::new()
        } else {
            if !opt.silent && opt.dump_filelist.is_none() && !opt.preprocess_only {
                let msg = format!(
                    "Config file '{}' is not found. Enable all rules",
                    opt.config.to_string_lossy()
                );
                printer.print_warning(&msg)?;
            }
            Config::new().enable_all()
        }
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
        linter.load(&plugin)?;
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

    let (files, incdirs) = if !opt.filelist.is_empty() {
        let mut files = opt.files.clone();
        let mut incdirs = opt.incdirs.clone();

        for filelist in &opt.filelist {
            let (mut f, mut i, d) = parse_filelist(filelist)?;
            if let Some(DumpFilelistMode::Yaml) = opt.dump_filelist {
                dump_filelist(printer, &DumpFilelistMode::Yaml, &filelist, &f, &i, &d)?;
            }
            files.append(&mut f);
            incdirs.append(&mut i);
            for (k, v) in d {
                defines.insert(k, v);
            }
        }

        (files, incdirs)
    } else {
        (opt.files.clone(), opt.incdirs.clone())
    };

    if let Some(mode) = &opt.dump_filelist {
        dump_filelist(printer, &mode, &Path::new("."), &files, &incdirs, &defines)?;
        return Ok(true);
    }

    let mut all_pass = true;

    for path in &files {
        let mut pass = true;
        if opt.preprocess_only {
            match preprocess(&path, &defines, &incdirs, false, opt.ignore_include) {
                Ok((text, new_defines)) => {
                    let msg = format!("{}", text.text());
                    printer.print(&msg)?;
                    defines = new_defines;
                }
                Err(x) => {
                    print_parser_error(printer, x, opt.oneline)?;
                    pass = false;
                }
            }
        } else {
            // Iterate over lines in the file, applying each textrule to each
            // line in turn.

            let text: String = read_to_string(&path)?;

            let mut beg: usize = 0;
            for line in text.split_inclusive('\n') {
                let line_stripped = line.trim_end_matches(&['\n', '\r']);

                for failed in linter.textrules_check(&line_stripped, &path, &beg) {
                    pass = false;
                    if !opt.silent {
                        printer.print_failed(&failed, opt.oneline, opt.github_actions)?;
                    }
                }
                beg += line.len();
            }

            match parse_sv_str(text.as_str(), &path, &defines, &incdirs, opt.ignore_include, false) {
                Ok((syntax_tree, new_defines)) => {
                    // Iterate over nodes in the concrete syntax tree, applying
                    // each syntaxrule to each node in turn.

                    for node in syntax_tree.into_iter().event() {
                        for failed in linter.syntaxrules_check(&syntax_tree, &node) {
                            pass = false;
                            if !opt.silent {
                                printer.print_failed(&failed, opt.oneline, opt.github_actions)?;
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
                    print_parser_error(printer, x, opt.oneline)?;
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
    oneline: bool,
) -> Result<(), Error> {
    match error {
        SvParserError::Parse(Some((path, pos))) => {
            printer.print_parse_error(&path, pos, oneline)?;
        }
        SvParserError::Preprocess(Some((path, pos))) => {
            printer.print_preprocess_error(&path, pos, oneline)?;
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
    };

    Ok(())
}

fn dump_completion<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
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
            .join("application")
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
            .join("application")
            .join("expected")
            .join(s);

        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        if cfg!(windows) {
            // 1. CRLF -> LF
            let ret = contents.replace("\r\n", "\n");

            // 2. "$CARGO_MANIFEST_DIR/foo/bar.baz" -> "$CARGO_MANIFEST_DIR\foo\bar.baz"
            let expected_paths = Regex::new(r"\$CARGO_MANIFEST_DIR[a-zA-Z0-9_/]+").unwrap();
            let mut r = ret.clone();
            for cap in expected_paths.captures_iter(&ret) {
                let expected_path = &cap[0];
                let runtime_path = expected_path.replace("/", "\\");
                r = r.replace(expected_path, &runtime_path);
            }
            let ret: String = r;

            // 3. "$CARGO_MANIFEST_DIR\foo\bar.baz" -> "C:\path\svlint\foo\bar.baz"
            let cargo_manifest_dir: String = cargo_manifest_dir.to_string();
            let ret = ret.replace("$CARGO_MANIFEST_DIR", cargo_manifest_dir.as_str());

            ret
        } else {
            contents.replace("$CARGO_MANIFEST_DIR", cargo_manifest_dir.as_str())
        }
    }

    fn textrules_test(rulename: &str, filename: &str, pass_not_fail: bool, silent: bool, oneline: bool) {
        let s = format!("[textrules]\n{} = true", rulename);
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

    fn syntaxrules_test(rulename: &str, filename: &str, pass_not_fail: bool, silent: bool, oneline: bool) {
        let s = format!("[syntaxrules]\n{} = true", rulename);
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

    #[test]
    fn dump_filelist_6() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Single deeper filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=files");
        args.push("--filelist");
        let f_1 = resources_path("grandparent1.fl");
        args.push(&f_1);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_6"));
    } // }}}

    #[test]
    fn dump_filelist_7() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Single deeper filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=incdirs");
        args.push("--filelist");
        let f_1 = resources_path("grandparent1.fl");
        args.push(&f_1);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_7"));
    } // }}}

    #[test]
    fn dump_filelist_8() {
        // {{{
        let config: Config = toml::from_str("").unwrap();

        // Single deeper filelist.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=defines");
        args.push("--filelist");
        let f_1 = resources_path("grandparent1.fl");
        args.push(&f_1);
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());
        assert_eq!(ret.unwrap(), true);

        let stdout = printer.read_to_string().unwrap();
        assert_eq!(stdout, expected_contents("dump_filelist_8"));
    } // }}}
}
