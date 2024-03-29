use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

const RENAMED_SYNTAXRULES: &[(&str, &str, &str)] = &[
    (
        "generate_keyword",
        "keyword_forbidden_generate",
        "KeywordForbiddenGenerate",
    ),
    ("tab_charactor", "tab_character", "TabCharacter"),
    (
        "genvar_declaration",
        "genvar_declaration_in_loop",
        "GenvarDeclarationInLoop",
    ),
    ("if_with_begin", "multiline_if_begin", "MultilineIfBegin"),
    ("for_with_begin", "multiline_for_begin", "MultilineForBegin"),
    (
        "legacy_always",
        "keyword_forbidden_always",
        "KeywordForbiddenAlways",
    ),
    (
        "generate_keyword_forbidden",
        "keyword_forbidden_generate",
        "KeywordForbiddenGenerate",
    ),
    (
        "priority_keyword",
        "keyword_forbidden_priority",
        "KeywordForbiddenPriority",
    ),
    (
        "unique_keyword",
        "keyword_forbidden_unique",
        "KeywordForbiddenUnique",
    ),
    (
        "unique0_keyword",
        "keyword_forbidden_unique0",
        "KeywordForbiddenUnique0",
    ),
    (
        "wire_reg",
        "keyword_forbidden_wire_reg",
        "KeywordForbiddenWireReg",
    ),
    (
        "generate_keyword_required",
        "keyword_required_generate",
        "KeywordRequiredGenerate",
    ),
    (
        "non_ansi_module",
        "module_nonansi_forbidden",
        "ModuleNonansiForbidden",
    ),
    (
        "level_sensitive_always",
        "general_always_no_edge",
        "GeneralAlwaysNoEdge",
    ),
];

fn write_rules_rs(
    textrules: &Vec<(String, String)>,
    syntaxrules: &Vec<(String, String)>,
) -> () {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let o = Path::new(&out_dir).join("rules.rs");
    let mut o = File::create(&o).unwrap();

    for (rulename, _) in textrules {
        let _ = writeln!(
            o,
            "#[path = \"{}/src/textrules/{}.rs\"]",
            root_dir.replace("\\", "\\\\"),
            rulename
        );
        let _ = writeln!(o, "pub mod {};", rulename);
    }

    for (rulename, _) in syntaxrules {
        let _ = writeln!(
            o,
            "#[path = \"{}/src/syntaxrules/{}.rs\"]",
            root_dir.replace("\\", "\\\\"),
            rulename
        );
        let _ = writeln!(o, "pub mod {};", rulename);
    }

    for (rulename, _) in textrules {
        let _ = writeln!(o, "pub use {}::*;", rulename);
    }

    for (rulename, _) in syntaxrules {
        let _ = writeln!(o, "pub use {}::*;", rulename);
    }
}

fn write_config_rules_rs(
    textrules: &Vec<(String, String)>,
    syntaxrules: &Vec<(String, String)>,
) -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let o = Path::new(&out_dir).join("config_rules.rs");
    let mut o = File::create(&o).unwrap();

    let _ = writeln!(o, "");
    let _ = writeln!(o, "#[derive(Clone, Debug, Deserialize, Serialize)]");
    let _ = writeln!(o, "#[serde(deny_unknown_fields)]");
    let _ = writeln!(o, "pub struct ConfigTextRules {{");

    for (rulename, _) in textrules {
        let _ = writeln!(o, "    #[serde(default = \"default_as_false\")]");
        let _ = writeln!(o, "    pub {}: bool,", rulename);
    }

    let _ = writeln!(o, "}}");

    let _ = writeln!(o, "");
    let _ = writeln!(o, "#[derive(Clone, Debug, Deserialize, Serialize)]");
    let _ = writeln!(o, "#[serde(deny_unknown_fields)]");
    let _ = writeln!(o, "pub struct ConfigSyntaxRules {{");

    for (rulename, _) in syntaxrules {
        let _ = writeln!(o, "    #[serde(default = \"default_as_false\")]");
        let _ = writeln!(o, "    pub {}: bool,", rulename);
    }

    let _ = writeln!(o, "");

    for (original_rulename, _, _) in RENAMED_SYNTAXRULES {
        let _ = writeln!(
            o,
            "    #[serde(default = \"default_as_false\", skip_serializing)]"
        );
        let _ = writeln!(o, "    pub {}: bool,", original_rulename);
    }

    let _ = writeln!(o, "}}");
}

fn write_impl_config_rs(
    textrules: &Vec<(String, String)>,
    syntaxrules: &Vec<(String, String)>,
) -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let o = Path::new(&out_dir).join("impl_config.rs");
    let mut o = File::create(&o).unwrap();

    let _ = writeln!(o, "");
    let _ = writeln!(o, "impl Config {{");
    let _ = writeln!(o, "    pub fn new() -> Self {{");
    let _ = writeln!(o, "        toml::from_str(\"\").unwrap()");
    let _ = writeln!(o, "    }}");

    // `enable_all()` used in `src/main.rs` when no config is found.
    let _ = writeln!(o, "");
    let _ = writeln!(o, "    pub fn enable_all(mut self) -> Self {{");
    for (rulename, _) in textrules {
        let _ = writeln!(o, "        self.textrules.{} = true;", rulename);
    }
    for (rulename, _) in syntaxrules {
        let _ = writeln!(o, "        self.syntaxrules.{} = true;", rulename);
    }
    let _ = writeln!(o, "");
    let _ = writeln!(o, "        self");
    let _ = writeln!(o, "    }}");

    // `gen_textrules()` used in `src/linter.rs` to gather rules to load.
    let _ = writeln!(o, "");
    let _ = writeln!(o, "    pub fn gen_textrules(&self) -> Vec<Box<dyn TextRule>> {{");
    let _ = writeln!(o, "        let mut ret: Vec<Box<dyn TextRule>> = Vec::new();");
    for (rulename, structname) in textrules {
        let _ = writeln!(o, "        if self.textrules.{} {{", rulename);
        let _ = writeln!(
            o,
            "            ret.push(Box::new({}::default()));",
            structname
        );
        let _ = writeln!(o, "        }}");
    }
    let _ = writeln!(o, "        ret");
    let _ = writeln!(o, "    }}");

    // `gen_syntaxrules()` used in `src/linter.rs` to gather rules to load.
    let _ = writeln!(o, "");
    let _ = writeln!(o, "    pub fn gen_syntaxrules(&self) -> Vec<Box<dyn SyntaxRule>> {{");
    let _ = writeln!(o, "        let mut ret: Vec<Box<dyn SyntaxRule>> = Vec::new();");
    for (rulename, structname) in syntaxrules {
        let _ = writeln!(o, "        if self.syntaxrules.{} {{", rulename);
        let _ = writeln!(
            o,
            "            ret.push(Box::new({}::default()));",
            structname
        );
        let _ = writeln!(o, "        }}");
    }
    for (original_rulename, _, structname) in RENAMED_SYNTAXRULES {
        let _ = writeln!(o, "        if self.syntaxrules.{} {{", original_rulename);
        let _ = writeln!(
            o,
            "            ret.push(Box::new({}::default()));",
            structname
        );
        let _ = writeln!(o, "        }}");
    }
    let _ = writeln!(o, "");
    let _ = writeln!(o, "        ret");
    let _ = writeln!(o, "    }}");

    // `gen_all_textrules()` used in `src/mdgen` to gather rules for MANUAL.
    let _ = writeln!(o, "");
    let _ = writeln!(o, "    pub fn gen_all_textrules() -> Vec<Box<dyn TextRule>> {{");
    let _ = writeln!(o, "        let mut ret: Vec<Box<dyn TextRule>> = Vec::new();");
    for (_, structname) in textrules {
        let _ = writeln!(o, "        ret.push(Box::new({}::default()));", structname);
    }
    let _ = writeln!(o, "");
    let _ = writeln!(o, "        ret");
    let _ = writeln!(o, "    }}");

    // `gen_all_syntaxrules()` used in `src/mdgen` to gather rules for MANUAL.
    let _ = writeln!(o, "");
    let _ = writeln!(o, "    pub fn gen_all_syntaxrules() -> Vec<Box<dyn SyntaxRule>> {{");
    let _ = writeln!(o, "        let mut ret: Vec<Box<dyn SyntaxRule>> = Vec::new();");
    for (_, structname) in syntaxrules {
        let _ = writeln!(o, "        ret.push(Box::new({}::default()));", structname);
    }
    let _ = writeln!(o, "");
    let _ = writeln!(o, "        ret");
    let _ = writeln!(o, "    }}");

    // `check_rename()` used in `src/main.rs` to detect whether a config uses
    // old names for rules which have been renamed.
    let _ = writeln!(o, "");
    let _ = writeln!(
        o,
        "    pub fn check_rename(&self) -> Vec<(String, String)> {{"
    );
    let _ = writeln!(
        o,
        "        let mut ret: Vec<(String, String)> = Vec::new();"
    );
    for (original_rulename, rulename, _) in RENAMED_SYNTAXRULES {
        let _ = writeln!(o, "        if self.syntaxrules.{} {{", original_rulename);
        let _ = writeln!(
            o,
            "            ret.push((String::from(\"{}\"), String::from(\"{}\")));",
            original_rulename, rulename
        );
        let _ = writeln!(o, "        }}");
    }
    let _ = writeln!(o, "");
    let _ = writeln!(o, "        ret");
    let _ = writeln!(o, "    }}");

    // `migrate()` used in `src/main.rs` to modify an in-memory config to use
    // new names (for rules which have been renamed).
    let _ = writeln!(o, "");
    let _ = writeln!(o, "    pub fn migrate(&mut self) {{");
    for (original_rulename, rulename, _) in RENAMED_SYNTAXRULES {
        let _ = writeln!(
            o,
            "        self.syntaxrules.{} = self.syntaxrules.{};",
            rulename, original_rulename
        );
    }
    let _ = writeln!(o, "    }}");

    let _ = writeln!(o, "}} // impl Config");
}

fn write_test_rs(
    textrules: &Vec<(String, String)>,
    syntaxrules: &Vec<(String, String)>,
) -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let o = Path::new(&out_dir).join("test.rs");
    let mut o = File::create(&o).unwrap();

    // blocking_assignment_in_always_ff testcases demonstrate comment control,
    // so visibility is useful when running `cargo test`.
    // E.g:
    //    let textrules_test_verbose = ["style_textwidth"];
    //    let syntaxrules_test_verbose = ["blocking_assignment_in_always_ff"];
    let textrules_test_verbose = [];
    let syntaxrules_test_verbose = [];

    for (rulename, _) in textrules {
        let silent = if textrules_test_verbose.contains(&rulename.as_str()) {
            "false"
        } else {
            "true"
        };

        for pass_not_fail in [true, false].iter() {
            let passfail = if *pass_not_fail { "pass" } else { "fail" };

            let test_filename = format!("testcases/textrules/{}/{}.sv", passfail, rulename);
            let lines = BufReader::new(File::open(test_filename).unwrap())
                .lines()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let sep = "/".repeat(80);
            let testcases: Vec<&[String]> = lines
                .as_slice()
                .split(|l| l.contains(sep.as_str()))
                .collect();
            let n_testcases: usize = testcases.len();

            for (t, testcase) in testcases.into_iter().enumerate().map(|(i, x)| (i + 1, x)) {
                // Write subtest to its own file.
                let subtest_path = Path::new(&out_dir)
                    .join(format!("textrules.{rulename}.{passfail}.{t}of{n_testcases}.sv"));
                let mut out_subtest = File::create(&subtest_path).unwrap();
                for line in testcase {
                    let _ = writeln!(out_subtest, "{}", line);
                }

                // Create call to `main.rs::tests::textrules_test()` via `tests.rs`.
                let subtest_name = format!("textrules_{rulename}_{passfail}_{t}of{n_testcases}");
                let _ = writeln!(o, "#[test]");
                let _ = writeln!(o, "fn {}() {{", subtest_name);
                if *pass_not_fail {
                    let _ = writeln!(
                        o,
                        "    textrules_test(\"{rulename}\", {subtest_path:?}, true, {silent}, false);"
                    );
                } else {
                    let _ = writeln!(
                        o,
                        "    textrules_test(\"{rulename}\", {subtest_path:?}, false, {silent}, false);"
                    );
                    let _ = writeln!(
                        o,
                        "    textrules_test(\"{rulename}\", {subtest_path:?}, false, {silent}, true);"
                    );
                }
                let _ = writeln!(o, "}}");
            }
        }
    }

    for (rulename, _) in syntaxrules {
        let silent = if syntaxrules_test_verbose.contains(&rulename.as_str()) {
            "false"
        } else {
            "true"
        };

        for pass_not_fail in [true, false].iter() {
            let passfail = if *pass_not_fail { "pass" } else { "fail" };

            let test_filename = format!("testcases/syntaxrules/{}/{}.sv", passfail, rulename);
            let lines = BufReader::new(File::open(test_filename).unwrap())
                .lines()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let sep = "/".repeat(80);
            let testcases: Vec<&[String]> = lines
                .as_slice()
                .split(|l| l.contains(sep.as_str()))
                .collect();
            let n_testcases: usize = testcases.len();

            for (t, testcase) in testcases.into_iter().enumerate().map(|(i, x)| (i + 1, x)) {
                // Write subtest to its own file.
                let subtest_path: std::path::PathBuf = Path::new(&out_dir)
                    .join(format!("syntaxrules_{rulename}_{passfail}_{t}of{n_testcases}.sv"));
                let mut out_subtest = File::create(&subtest_path).unwrap();
                for line in testcase {
                    let _ = writeln!(out_subtest, "{}", line);
                }

                // Create call to `main.rs::tests::syntaxrules_test()` via `tests.rs`.
                let subtest_name = format!("syntaxrules_{rulename}_{passfail}_{t}of{n_testcases}");
                let _ = writeln!(o, "#[test]");
                let _ = writeln!(o, "fn {}() {{", subtest_name);
                if *pass_not_fail {
                    let _ = writeln!(
                        o,
                        "    syntaxrules_test(\"{rulename}\", {subtest_path:?}, true, {silent}, false);"
                    );
                } else {
                    let _ = writeln!(
                        o,
                        "    syntaxrules_test(\"{rulename}\", {subtest_path:?}, false, {silent}, false);"
                    );
                    let _ = writeln!(
                        o,
                        "    syntaxrules_test(\"{rulename}\", {subtest_path:?}, false, {silent}, true);"
                    );
                }
                let _ = writeln!(o, "}}");
            }
        }
    }
}

fn main() {
    let re_struct = Regex::new(r"pub struct ([a-zA-Z0-9]*)").unwrap();

    let mut syntaxrules = Vec::new();
    for entry in WalkDir::new("src/syntaxrules") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let f = File::open(entry.path()).unwrap();
            let mut f = BufReader::new(f);
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            let cap = re_struct.captures(&s);
            if let Some(cap) = cap {
                let struct_name = String::from(&cap[1]);
                let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
                syntaxrules.push((file_name, struct_name));
            }
        }
    }

    syntaxrules.sort_by(|a, b| a.0.cmp(&b.0));

    let mut textrules = Vec::new();
    for entry in WalkDir::new("src/textrules") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let f = File::open(entry.path()).unwrap();
            let mut f = BufReader::new(f);
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            let cap = re_struct.captures(&s);
            if let Some(cap) = cap {
                let struct_name = String::from(&cap[1]);
                let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
                textrules.push((file_name, struct_name));
            }
        }
    }

    textrules.sort_by(|a, b| a.0.cmp(&b.0));

    write_rules_rs(&textrules, &syntaxrules);
    write_config_rules_rs(&textrules, &syntaxrules);
    write_impl_config_rs(&textrules, &syntaxrules);
    write_test_rs(&textrules, &syntaxrules);
}
