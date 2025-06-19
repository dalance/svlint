use anyhow::{Context, Error};
use chardetng::EncodingDetector;
use clap::{Parser, CommandFactory};
use clap_complete;
use enquote;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, process};
use sv_filelist_parser;
use sv_parser::Error as SvParserError;
use sv_parser::{parse_sv_str, preprocess, Define, DefineText};
use svlint::config::Config;
use svlint::linter::{Linter, TextRuleEvent};
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
    #[clap(required_unless_present_any = &["filelists", "config-example", "config-update", "shell-completion"])]
    pub files: Vec<PathBuf>,

    /// Filelist file(s)
    #[clap(short = 'f', long = "filelist", conflicts_with = "files")]
    pub filelists: Vec<PathBuf>,

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
    #[clap(value_enum, long = "shell-completion")]
    pub shell_completion: Option<clap_complete::Shell>,

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

    if let Some(generator) = opt.shell_completion {
        let mut cmd = Opt::command();
        shell_completion(generator, &mut cmd);
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

    let (files, incdirs) = if !opt.filelists.is_empty() {
        let mut files = opt.files.clone();
        let mut incdirs = opt.incdirs.clone();

        for filelist in &opt.filelists {
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

        get_files_incdirs(files, incdirs)
    } else {
        get_files_incdirs(opt.files.clone(), opt.incdirs.clone())
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

            // Signal beginning of file to all TextRules, which *may* be used
            // by textrules to reset their internal state.
            let _ = linter.textrules_check(TextRuleEvent::StartOfFile, &path, &0);

            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer)?;
            let mut detector = EncodingDetector::new();
            detector.feed(&buffer, true);
            let encoding = detector.guess(None, true).decode(&buffer).0;

            let text = encoding.into_owned();
            let mut beg: usize = 0;

            // Iterate over lines in the file, applying each textrule to each
            // line in turn.
            for line in text.split_inclusive('\n') {
                let line_stripped = line.trim_end_matches(&['\n', '\r']);

                for failed in linter.textrules_check(TextRuleEvent::Line(&line_stripped), &path, &beg) {
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

        if opt.verbose {
            printer.print_info(&format!(
                "{} '{}'",
                if pass { "pass" } else { "fail" },
                path.display()
            ))?;
        }
        all_pass &= pass;
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
        SvParserError::Include { source } => match *source {
            SvParserError::File { source: _, path } => {
                printer.print_error(&format!("failed to include '{}'", path.display()))?;
            }
            SvParserError::DefineNotFound(define) => {
                printer.print_error(&format!("definition not found for '{}'", define))?;
            }
            _ => {
                printer.print_error(&format!("{}", source))?;
            }
        },
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
fn get_files_incdirs(
    cli_files: Vec<PathBuf>,
    cli_incdirs: Vec<PathBuf>
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let env_incdirs: Vec<PathBuf> = if let Ok(e) = env::var("SVLINT_INCDIRS") {
        env::split_paths(&e).map(|x| PathBuf::from(x)).collect()
    } else {
        vec![]
    };

    let env_prefiles: Vec<PathBuf> = if let Ok(e) = env::var("SVLINT_PREFILES") {
        env::split_paths(&e).map(|x| PathBuf::from(x)).collect()
    } else {
        vec![]
    };

    let env_postfiles: Vec<PathBuf> = if let Ok(e) = env::var("SVLINT_POSTFILES") {
        env::split_paths(&e).map(|x| PathBuf::from(x)).collect()
    } else {
        vec![]
    };

    let ret_files: Vec<PathBuf> = [env_prefiles, cli_files, env_postfiles].concat();
    let ret_incdirs: Vec<PathBuf> = [env_incdirs, cli_incdirs].concat();

    (ret_files, ret_incdirs)
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

fn shell_completion<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
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
    #[allow(unused_variables)]
    fn cli_oneline() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-1");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--oneline");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_config() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-c");
        args.push("foo.toml");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--config");
        args.push("foo.toml");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_config_example() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--config-example");
        let opt = Opt::parse_from(args.iter());

        // Alias for backwards compatibility svlint v0.8.0 and earlier. ////////
        let mut args = vec!["svlint"];
        args.push("--example");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_config_update() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--config-update");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--config");
        args.push("foo.toml");
        args.push("--config-update");
        let opt = Opt::parse_from(args.iter());

        // Alias for backwards compatibility svlint v0.8.0 and earlier. ////////
        let mut args = vec!["svlint"];
        args.push("--update");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_defines() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-D");
        args.push("FOO");
        args.push("-D");
        args.push("BAR");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-D");
        args.push("FOO=123");
        args.push("-D");
        args.push("BAR=456");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-DFOO");
        args.push("-DBAR");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-DFOO=123");
        args.push("-DBAR=456");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        // Long option. ////////////////////////////////////////////////////////
        let mut args = vec!["svlint"];
        args.push("--define");
        args.push("FOO");
        args.push("--define");
        args.push("BAR");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--define");
        args.push("FOO=123");
        args.push("--define");
        args.push("BAR=456");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        // Alias for backwards compatibility svlint v0.8.0 and earlier. ////////
        let mut args = vec!["svlint"];
        args.push("-d");
        args.push("FOO");
        args.push("-d");
        args.push("BAR");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-d");
        args.push("FOO=123");
        args.push("-d");
        args.push("BAR=456");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-dFOO");
        args.push("-dBAR");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-dFOO=123");
        args.push("-dBAR=456");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_shell_completion() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--shell-completion=bash");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--shell-completion=elvish");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--shell-completion=fish");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--shell-completion=powershell");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--shell-completion=zsh");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_dump_filelist() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("--filelist");
        args.push("foo.f");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--dump-filelist=files");
        args.push("--filelist");
        args.push("foo.f");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--filelist");
        args.push("foo.f");
        args.push("--dump-filelist=incdirs");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--filelist");
        args.push("foo.f");
        args.push("--dump-filelist=defines");

        // Without the -f/--filelist. //////////////////////////////////////////
        // Useful for debugging other ways of passing long/complex commands.
        let mut args = vec!["svlint"];
        args.push("--dump-filelist=yaml");
        args.push("-DFOO");
        args.push("-Ipath/to/headers/");
        args.push("foo.sv");
        args.push("bar.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--dump-filelist=files");
        args.push("-DFOO");
        args.push("-Ipath/to/headers/");
        args.push("foo.sv");
        args.push("bar.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--dump-filelist=incdirs");
        args.push("-DFOO");
        args.push("-Ipath/to/headers/");
        args.push("foo.sv");
        args.push("bar.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--dump-filelist=defines");
        args.push("-DFOO");
        args.push("-Ipath/to/headers/");
        args.push("foo.sv");
        args.push("bar.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_dump_syntaxtree() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--dump-syntaxtree");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_preprocess_only() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-E");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--preprocess-only");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_filelist() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-f");
        args.push("Foo.f");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-fFoo.f");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--filelist");
        args.push("foo.f");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_github_actions() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--github-actions");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    // NOTE: Testing clap's -h/--help interfers with `cargo test`.

    #[test]
    #[allow(unused_variables)]
    fn cli_incdirs() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-I");
        args.push("path/to/foo");
        args.push("-I");
        args.push("/path/to/bar");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-Ipath/to/foo");
        args.push("-I/path/to/bar");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--incdir");
        args.push("path/to/foo");
        args.push("--incdir");
        args.push("/path/to/bar");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        // Aliases for backwards compatibility svlint v0.8.0 and earlier. //////
        let mut args = vec!["svlint"];
        args.push("-i");
        args.push("path/to/foo");
        args.push("-i");
        args.push("/path/to/bar");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-ipath/to/foo");
        args.push("-i/path/to/bar");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--include");
        args.push("path/to/foo");
        args.push("--include");
        args.push("/path/to/bar");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_ignore_include() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("--ignore-include");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_plugins() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-p");
        args.push("path/to/libfoo.so"); // Linux
        args.push("-p");
        args.push("path/to/libbar.so");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-p");
        args.push("path/to/libfoo.dylib"); // MacOS
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-p");
        args.push("path\\to\\foo.dll"); // Windows
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("-pfoo.so");
        args.push("-pbar.so");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--plugin");
        args.push("foo.so");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_silent() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-s");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--silent");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    #[test]
    #[allow(unused_variables)]
    fn cli_verbose() {
        // {{{
        let mut args = vec!["svlint"];
        args.push("-v");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());

        let mut args = vec!["svlint"];
        args.push("--verbose");
        args.push("foo.sv");
        let opt = Opt::parse_from(args.iter());
    } // }}}

    // NOTE: Testing clap's -V/--version interfers with `cargo test`.

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

    #[test]
    fn lint_gbk_encoded_verilog() {
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        // Create a temporary Verilog file
        let temp_path = Path::new("temp_gbk_verilog.sv");
        let mut file = File::create(&temp_path).expect("Failed to create test file");

        // Write GBK-encoded Verilog code
        let gbk_verilog = vec![
            0x0a, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x70, 0x3b, 0x0a, 0x2f,
            0x2f, 0x20, 0xd6, 0xd0, 0xce, 0xc4, 0xd7, 0xa2, 0xca, 0xcd, 0x0a, 0x65, 0x6e, 0x64,
            0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x0a,
        ];
        file.write_all(&gbk_verilog)
            .expect("Failed to write test file");
        drop(file); // Close the file

        // Run `svlint` to analyze the file
        let config: Config = toml::from_str("").unwrap();
        let mut args = vec!["svlint"];
        args.push(temp_path.to_str().unwrap());
        let opt = Opt::parse_from(args.iter());

        let mut printer = Printer::new(true);
        let ret = run_opt_config(&mut printer, &opt, config.clone());

        // Clean up the test file
        std::fs::remove_file(&temp_path).expect("Failed to remove test file");

        // Assert that `svlint` successfully processes the GBK-encoded Verilog file
        assert!(ret.is_ok(), "svlint failed to process GBK-encoded Verilog");
    }
}
