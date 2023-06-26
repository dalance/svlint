#![allow(dead_code)]

mod config;
mod linter;
mod printer;
mod rules;

use crate::config::{Config, ConfigOption};
use crate::linter::{TextRule, SyntaxRule};
use regex::Regex;
use std::env;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader, Read, Write, Error};
use std::path::Path;

struct Ruleset {
    name: String,
    md: Vec<String>,
    sh: Vec<String>,
    cmd: Vec<String>,
    toml: Vec<String>,
}

fn file_contents(path: &str) -> String {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&cargo_manifest_dir).join(path);

    let file: File = File::open(path).unwrap();
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents
}

fn get_rulesets() -> Vec<Ruleset> {
    let mut definitions: Vec<(String, String)> = Vec::new();
    let re: Regex = Regex::new(r"ruleset-([a-zA-Z0-9_-]+)\.md").unwrap();
    let entries = read_dir("md").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>().unwrap();
    for entry in entries {
        let entry = entry.to_str().unwrap();
        if let Some(caps) = re.captures(entry) {
            let name = String::from(caps.get(1).unwrap().as_str());
            definitions.push((name, file_contents(&entry)));
        }
    }
    definitions.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    let mut ret = Vec::new();
    for (name, contents) in definitions {
        enum DefinitionLineState {
            Markdown,
            PosixShell,
            WindowsBatch,
            TomlConfig,
        }

        let mut linestate = DefinitionLineState::Markdown;
        let mut md: Vec<String> = Vec::new();
        let mut sh: Vec<String> = Vec::new();
        let mut cmd: Vec<String> = Vec::new();
        let mut toml: Vec<String> = Vec::new();

        for line in contents.lines() {
            if line.starts_with("```toml") {
                linestate = DefinitionLineState::TomlConfig;
            } else if line.starts_with("```winbatch") {
                linestate = DefinitionLineState::WindowsBatch;
            } else if line.starts_with("```sh") {
                linestate = DefinitionLineState::PosixShell;
            } else if line.starts_with("```") {
                linestate = DefinitionLineState::Markdown;
            } else {
                let line = line.to_string();
                let _ = match linestate {
                    DefinitionLineState::Markdown => md.push(line),
                    DefinitionLineState::PosixShell => sh.push(line),
                    DefinitionLineState::WindowsBatch => cmd.push(line),
                    DefinitionLineState::TomlConfig => toml.push(line),
                };
            }
        }

        ret.push(Ruleset {
            name: name,
            md: md,
            sh: sh,
            cmd: cmd,
            toml: toml,
        });
    }

    ret
}

fn write_md_rule_testcases(o: &mut File, ruletype: &str, rulename: &str, pass_not_fail: bool) -> () {
        let sep = "/".repeat(80);

        let passfail = if pass_not_fail { "pass" } else { "fail" };
        let filename = format!("testcases/{}rules/{}/{}.sv", ruletype, passfail, rulename);
        let lines = BufReader::new(File::open(filename).unwrap())
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let testcases: Vec<&[String]> = lines
            .as_slice()
            .split(|l| l.contains(sep.as_str()))
            .collect();
        let n_testcases: usize = testcases.len();

        let passfail = if pass_not_fail { "Pass" } else { "Fail" };
        for (t, testcase) in testcases.into_iter().enumerate().map(|(i, x)| (i + 1, x)) {
            let _ = writeln!(o, "### {passfail} Example ({t} of {n_testcases})");
            let _ = writeln!(o, "```systemverilog");
            for line in testcase {
                let _ = writeln!(o, "{}", line);
            }
            let _ = writeln!(o, "```");
            let _ = writeln!(o, "");
        }
}

fn write_md_textrules(o: &mut File, textrules: Vec<Box<dyn TextRule>>) -> () {
    for rule in textrules {
        let rulename = rule.name();

        let _ = writeln!(o, "");
        let _ = writeln!(o, "* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "## Text Rule: `{}`\n", rulename);

        let _ = writeln!(o, "### Hint\n");
        let _ = writeln!(o, "{}\n", rule.hint(&ConfigOption::default()));

        let _ = writeln!(o, "### Reason\n");
        let _ = writeln!(o, "{}\n", rule.reason());

        write_md_rule_testcases(o, "text", &rulename, true);
        write_md_rule_testcases(o, "text", &rulename, false);

        let _ = writeln!(o, "### Explanation\n");
        let p: String = format!("md/textrules-explanation-{}.md", rule.name());
        let _ = writeln!(o, "{}\n", file_contents(&p));
    }
}

fn write_md_syntaxrules(o: &mut File, syntaxrules: Vec<Box<dyn SyntaxRule>>) -> () {
    for rule in syntaxrules {
        let rulename = rule.name();

        let _ = writeln!(o, "");
        let _ = writeln!(o, "* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "## Syntax Rule: `{}`\n", rule.name());

        let _ = writeln!(o, "### Hint\n");
        let _ = writeln!(o, "{}\n", rule.hint(&ConfigOption::default()));

        let _ = writeln!(o, "### Reason\n");
        let _ = writeln!(o, "{}\n", rule.reason());

        write_md_rule_testcases(o, "syntax", &rulename, true);
        write_md_rule_testcases(o, "syntax", &rulename, false);

        let _ = writeln!(o, "### Explanation\n");
        let p: String = format!("md/syntaxrules-explanation-{}.md", rule.name());
        let _ = writeln!(o, "{}\n", file_contents(&p));
    }
}

fn partition_syntaxrules(
    syntaxrules: Vec<Box<dyn SyntaxRule>>,
) -> (Vec<Box<dyn SyntaxRule>>, Vec<Box<dyn SyntaxRule>>, Vec<Box<dyn SyntaxRule>>) {
    let style_prefixes = ["style_", "tab_"].join("|");
    let re_style: Regex = Regex::new(format!("^({})", style_prefixes).as_str()).unwrap();

    let naming_prefixes = ["prefix_", "lowercamelcase_", "uppercamelcase_", "re_"].join("|");
    let re_naming: Regex =
        Regex::new(format!("(^({})|_with_label$)", naming_prefixes).as_str()).unwrap();

    let mut part_style: Vec<Box<dyn SyntaxRule>> = Vec::new();
    let mut part_naming: Vec<Box<dyn SyntaxRule>> = Vec::new();
    let mut part_functional: Vec<Box<dyn SyntaxRule>> = Vec::new();

    for rule in syntaxrules {
        if re_style.is_match(&rule.name()) {
            part_style.push(rule);
        } else if re_naming.is_match(&rule.name()) {
            part_naming.push(rule);
        } else {
            part_functional.push(rule);
        }
    }

    (part_functional, part_naming, part_style)
}

fn write_manual_md(
    textrules: Vec<Box<dyn TextRule>>,
    syntaxrules: Vec<Box<dyn SyntaxRule>>,
    rulesets: Vec<Ruleset>,
) -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let o = Path::new(&cargo_manifest_dir).join("MANUAL.md");
    let mut o = File::create(&o).unwrap();

    let (syntaxrules_functional, syntaxrules_naming, syntaxrules_style) = partition_syntaxrules(syntaxrules);

    let _ = writeln!(
        o,
        "{}",
        file_contents(format!("md/manual-introduction.md").as_str())
    );

    let _ = writeln!(
        o,
        "{}",
        file_contents(format!("md/manual-textrules.md").as_str())
    );
    write_md_textrules(&mut o, textrules);

    let _ = writeln!(
        o,
        "{}",
        file_contents(format!("md/manual-syntaxrules-functional.md").as_str())
    );
    write_md_syntaxrules(&mut o, syntaxrules_functional);

    let _ = writeln!(
        o,
        "{}",
        file_contents(format!("md/manual-syntaxrules-naming.md").as_str())
    );
    write_md_syntaxrules(&mut o, syntaxrules_naming);

    let _ = writeln!(
        o,
        "{}",
        file_contents(format!("md/manual-syntaxrules-style.md").as_str())
    );
    write_md_syntaxrules(&mut o, syntaxrules_style);

    let _ = writeln!(o, "");
    let _ = writeln!(o, "* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *");
    let _ = writeln!(o, "");
    let _ = writeln!(o, "* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *");
    let _ = writeln!(o, "");

    let _ = writeln!(
        o,
        "{}",
        file_contents(format!("md/manual-rulesets.md").as_str())
    );
    for ruleset in &rulesets {
        let _ = writeln!(o, "");
        let _ = writeln!(o, "* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "## Ruleset: *{}*", &ruleset.name);
        let _ = writeln!(
            o,
            "{}",
            file_contents(format!("md/ruleset-{}.md", &ruleset.name).as_str())
        );
    }
}

fn write_ruleset_sh_svlint(ruleset: &Ruleset) -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let p = Path::new(&cargo_manifest_dir)
        .join("rulesets")
        .join(format!("svlint-{}", ruleset.name));

    {
        let mut o = File::create(&p).unwrap();

        let _ = writeln!(o, "#!/usr/bin/env sh");
        let _ = writeln!(o, "set -e");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "# If flag/options are given that don't use the ruleset config, simply run");
        let _ = writeln!(o, "# svlint with the given arguments.");
        let _ = writeln!(o, "NONRULESET=\"-h|--help|-V|--version|--dump-filelist|-E|--example|--update\"");
        let _ = writeln!(o, "if printf \"%b\\n\" \" $*\" | grep -Eq \" (${{NONRULESET}})\";");
        let _ = writeln!(o, "then");
        let _ = writeln!(o, "  svlint $*");
        let _ = writeln!(o, "  exit $?");
        let _ = writeln!(o, "fi");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "SVLINT_CONFIG=\"$(dirname $(command -v svlint-{0}))/{0}.toml\"", ruleset.name);
        let _ = writeln!(o, "");
        let _ = writeln!(o, "# Delete ANSI control sequences that begin with ESC and (usually) end with m.");
        let _ = writeln!(o, "# Delete ASCII control characters except line feed ('\\n' = 0o12 = 10 = 0x0A).");
        let _ = writeln!(o, "SANS_CONTROL=\"| sed -e 's/\\\\o33\\\\[[0-9;]*[mGKHF]//g'\"");
        let _ = writeln!(o, "SANS_CONTROL=\"${{SANS_CONTROL}} | tr -d '[\\\\000-\\\\011\\\\013-\\\\037\\\\177]'\"");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "# Combine the above output sanitization fragments into variables which can be");
        let _ = writeln!(o, "# evaluated and processed with xargs, e.g:");
        let _ = writeln!(o, "#   eval \"${{SVFILES}}\" | xargs -I {{}} sh -c \"grep foo {{}};\"");
        let _ = writeln!(o, "# NOTE: Creating a variable with the result (instead of the command) would lead");
        let _ = writeln!(o, "# to undefined behavior where the list of file paths exceeds 2MiB.");
        let _ = writeln!(o, "SVFILES=\"svlint --dump-filelist=files $* ${{SANS_CONTROL}}\"");
        let _ = writeln!(o, "SVINCDIRS=\"svlint --dump-filelist=incdirs $* ${{SANS_CONTROL}}\"");
        let _ = writeln!(o, "");
        for line in &ruleset.sh {
            let _ = writeln!(o, "{}", line);
        }
        let _ = writeln!(o, "");
        let _ = writeln!(o, "env SVLINT_CONFIG=\"${{SVLINT_CONFIG}}\" svlint $*");
    }

    #[cfg(unix)]
    {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;
        set_permissions(&p, Permissions::from_mode(0o755)).unwrap();
    }
}

fn write_ruleset_sh_svls(ruleset: &Ruleset) -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let p = Path::new(&cargo_manifest_dir)
        .join("rulesets")
        .join(format!("svls-{}", ruleset.name));

    {
        let mut o = File::create(&p).unwrap();

        let _ = writeln!(o, "#!/usr/bin/env sh");
        let _ = writeln!(o, "set -e");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "# If flag/options are given that don't use the ruleset config, simply run");
        let _ = writeln!(o, "# svls with the given arguments.");
        let _ = writeln!(o, "NONRULESET=\"-h|--help|-V|--version\"");
        let _ = writeln!(o, "if printf \"%b\\n\" \" $*\" | grep -Eq \" (${{NONRULESET}})\";");
        let _ = writeln!(o, "then");
        let _ = writeln!(o, "  svls $*");
        let _ = writeln!(o, "  exit $?");
        let _ = writeln!(o, "fi");
        let _ = writeln!(o, "");
        let _ = writeln!(o, "SVLINT_CONFIG=\"$(dirname $(command -v svls-{0}))/{0}.toml\"", ruleset.name);
        let _ = writeln!(o, "");
        let _ = writeln!(o, "env SVLINT_CONFIG=\"${{SVLINT_CONFIG}}\" svls $*");
    }

    #[cfg(unix)]
    {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;
        set_permissions(&p, Permissions::from_mode(0o755)).unwrap();
    }
}

fn write_ruleset_cmd_svlint(ruleset: &Ruleset) -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let p = Path::new(&cargo_manifest_dir)
        .join("rulesets")
        .join(format!("svlint-{}.cmd", ruleset.name));
    let mut o = File::create(&p).unwrap();

    let _ = write!(o, "\r\n");
    let _ = write!(o, "@echo off\r\n");
    let _ = write!(o, "for /f %%E in ('where.exe svlint-{0}') do (\r\n", ruleset.name);
    let _ = write!(o, "    set \"SVLINT_CONFIG=%%~dpE{0}.toml\"\r\n", ruleset.name);
    let _ = write!(o, ")\r\n");
    for line in &ruleset.cmd {
        let _ = write!(o, "{}\r\n", line);
    }
    let _ = write!(o, "svlint %*\r\n");
    let _ = write!(o, "\r\n");
}

fn write_ruleset_cmd_svls(ruleset: &Ruleset) -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let p = Path::new(&cargo_manifest_dir)
        .join("rulesets")
        .join(format!("svls-{}.cmd", ruleset.name));
    let mut o = File::create(&p).unwrap();

    let _ = write!(o, "\r\n");
    let _ = write!(o, "@echo off\r\n");
    let _ = write!(o, "for /f %%E in ('where.exe svls-{0}') do (\r\n", ruleset.name);
    let _ = write!(o, "    set \"SVLINT_CONFIG=%%~dpE{0}.toml\"\r\n", ruleset.name);
    let _ = write!(o, ")\r\n");
    let _ = write!(o, "svls %*\r\n");
    let _ = write!(o, "\r\n");
}

fn write_ruleset_toml(ruleset: &Ruleset) -> () {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let p = Path::new(&cargo_manifest_dir)
        .join("rulesets")
        .join(format!("{}.toml", ruleset.name));
    let mut o = File::create(&p).unwrap();

    for line in &ruleset.toml {
        let _ = writeln!(o, "{}", line);
    }
}


#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let rulesets = get_rulesets();
    for ruleset in &rulesets {
        write_ruleset_sh_svlint(ruleset);
        write_ruleset_sh_svls(ruleset);
        write_ruleset_cmd_svlint(ruleset);
        write_ruleset_cmd_svls(ruleset);
        write_ruleset_toml(ruleset);
    }

    let textrules = Config::gen_all_textrules();
    let syntaxrules = Config::gen_all_syntaxrules();
    write_manual_md(textrules, syntaxrules, rulesets);
}
