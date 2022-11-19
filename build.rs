use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

const RENAMED_RULES: &[(&str, &str, &str)] = &[
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
];

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let re_struct = Regex::new(r"pub struct ([a-zA-Z0-9]*)").unwrap();

    let mut rules = Vec::new();
    for entry in WalkDir::new("src/rules") {
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
                rules.push((file_name, struct_name));
            }
        }
    }

    rules.sort_by(|a, b| a.0.cmp(&b.0));

    // -------------------------------------------------------------------------------------------------
    // Output 'rules.rs'
    // -------------------------------------------------------------------------------------------------

    let out_rules = Path::new(&out_dir).join("rules.rs");
    let mut out_rules = File::create(&out_rules).unwrap();

    for (file_name, _) in &rules {
        let _ = write!(
            out_rules,
            "#[path = \"{}/src/rules/{}.rs\"]\n",
            root_dir.replace("\\", "\\\\"),
            file_name
        );
        let _ = write!(out_rules, "pub mod {};\n", file_name);
    }
    for (file_name, _) in &rules {
        let _ = write!(out_rules, "pub use {}::*;\n", file_name);
    }

    // -------------------------------------------------------------------------------------------------
    // Output 'config_rules.rs'
    // -------------------------------------------------------------------------------------------------

    let out_config_rules = Path::new(&out_dir).join("config_rules.rs");
    let mut out_config_rules = File::create(&out_config_rules).unwrap();

    let mut body = String::new();
    for (file_name, _) in &rules {
        body.push_str(&format!("    #[serde(default = \"default_as_false\")]\n"));
        body.push_str(&format!("    pub {}: bool,\n", file_name));
    }
    for (org_name, _, _) in RENAMED_RULES {
        body.push_str(&format!(
            "    #[serde(default = \"default_as_false\", skip_serializing)]\n"
        ));
        body.push_str(&format!("    pub {}: bool,\n", org_name));
    }

    let str_config_rules = format!(
        r##"
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigRules {{
{}
}}"##,
        body
    );
    let _ = write!(out_config_rules, "{}", str_config_rules);

    // -------------------------------------------------------------------------------------------------
    // Output 'impl_config.rs'
    // -------------------------------------------------------------------------------------------------

    let out_impl_config = Path::new(&out_dir).join("impl_config.rs");
    let mut out_impl_config = File::create(&out_impl_config).unwrap();

    let mut enable_all_body = String::new();
    let mut gen_rules_body = String::new();
    let mut gen_all_rules_body = String::new();
    let mut check_rename_body = String::new();
    let mut migrate_body = String::new();
    for (file_name, struct_name) in &rules {
        enable_all_body.push_str(&format!("        self.rules.{} = true;\n", file_name));
        gen_rules_body.push_str(&format!("        if self.rules.{} {{\n", file_name));
        gen_rules_body.push_str(&format!(
            "            ret.push(Box::new({}::default()));\n",
            struct_name
        ));
        gen_rules_body.push_str(&format!("        }}\n"));
        gen_all_rules_body.push_str(&format!(
            "        ret.push(Box::new({}::default()));\n",
            struct_name
        ));
    }
    for (org_name, file_name, struct_name) in RENAMED_RULES {
        gen_rules_body.push_str(&format!("        if self.rules.{} {{\n", org_name));
        gen_rules_body.push_str(&format!(
            "            ret.push(Box::new({}::default()));\n",
            struct_name
        ));
        gen_rules_body.push_str(&format!("        }}\n"));
        check_rename_body.push_str(&format!("        if self.rules.{} {{\n", org_name));
        check_rename_body.push_str(&format!(
            "            ret.push((String::from(\"{}\"), String::from(\"{}\")));\n",
            org_name, file_name
        ));
        check_rename_body.push_str(&format!("        }}\n"));
        migrate_body.push_str(&format!(
            "        self.rules.{} = self.rules.{};\n",
            file_name, org_name
        ));
    }

    let str_impl_config = format!(
        r##"
impl Config {{
    pub fn new() -> Self {{
        toml::from_str("").unwrap()
    }}

    pub fn enable_all(mut self) -> Self {{
{}
        self
    }}

    pub fn gen_rules(&self) -> Vec<Box<dyn Rule>> {{
        let mut ret: Vec<Box<dyn Rule>> = Vec::new();
{}
        ret
    }}

    pub fn gen_all_rules() -> Vec<Box<dyn Rule>> {{
        let mut ret: Vec<Box<dyn Rule>> = Vec::new();
{}
        ret
    }}

    pub fn check_rename(&self) -> Vec<(String, String)> {{
        let mut ret: Vec<(String, String)> = Vec::new();
{}
        ret
    }}

    pub fn migrate(&mut self) {{
{}
    }}
}}"##,
        enable_all_body, gen_rules_body, gen_all_rules_body, check_rename_body, migrate_body
    );
    let _ = write!(out_impl_config, "{}", str_impl_config);

    // -------------------------------------------------------------------------------------------------
    // Output 'test.rs'
    // -------------------------------------------------------------------------------------------------

    let out_test = Path::new(&out_dir).join("test.rs");
    let mut out_test = File::create(&out_test).unwrap();

    // blocking_assignment_in_always_ff testcases demonstrate comment control,
    // so visibility is useful when running `cargo test`.
    let test_verbose = ["blocking_assignment_in_always_ff"];

    for (rulename, _) in &rules {
        let silent = if test_verbose.contains(&rulename.as_str()) {
            "false"
        } else {
            "true"
        };

        for pass_not_fail in [true, false].iter() {
            let passfail = if *pass_not_fail {
                "pass"
            } else {
                "fail"
            };

            let test_filename = format!("testcases/{}/{}.sv", passfail, rulename);
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

            for (t, testcase) in testcases
                .into_iter()
                .enumerate()
                .map(|(i, x)| (i + 1, x)) {
                let subtest_name = format!("{rulename}_{passfail}_{t}of{n_testcases}");
                let subtest_path = Path::new(&out_dir).join(format!("{rulename}.{passfail}.{t}of{n_testcases}.sv"));
                let p = subtest_path.display();

                let _ = write!(out_test, "#[test]\n");
                let _ = write!(out_test, "fn {}() {{\n", subtest_name);
                if *pass_not_fail {
                    let _ = write!(out_test, "    test(\"{rulename}\", \"{p}\", true, {silent}, false);\n");
                } else {
                    let _ = write!(out_test, "    test(\"{rulename}\", \"{p}\", false, {silent}, false);\n");
                    let _ = write!(out_test, "    test(\"{rulename}\", \"{p}\", false, {silent}, true);\n");
                }
                let _ = write!(out_test, "}}\n");

                // Write subtest to its own file.
                let out_subtest = subtest_path;
                let mut out_subtest = File::create(&out_subtest).unwrap();
                for line in testcase {
                    let _ = write!(out_subtest, "{}\n", line);
                }
            }
        }
    }
}
