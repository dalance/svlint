use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let re_struct = Regex::new(r"pub struct ([a-zA-Z0-9]*);").unwrap();

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

    // Output 'rules.rs'
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

    // Output 'config_rules.rs'
    let out_config_rules = Path::new(&out_dir).join("config_rules.rs");
    let mut out_config_rules = File::create(&out_config_rules).unwrap();

    let mut body = String::new();
    for (file_name, _) in &rules {
        body.push_str(&format!("    #[serde(default = \"default_as_false\")]\n"));
        body.push_str(&format!("    pub {}: bool,\n", file_name));
    }

    let str_config_rules = format!(
        r##"
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigRules {{
{}
}}"##,
        body
    );
    let _ = write!(out_config_rules, "{}", str_config_rules);

    // Output 'impl_config.rs'
    let out_impl_config = Path::new(&out_dir).join("impl_config.rs");
    let mut out_impl_config = File::create(&out_impl_config).unwrap();

    let mut body = String::new();
    let mut body_all = String::new();
    for (file_name, struct_name) in &rules {
        body.push_str(&format!("        if self.rules.{} {{\n", file_name));
        body.push_str(&format!(
            "            ret.push(Box::new({}));\n",
            struct_name
        ));
        body.push_str(&format!("        }}\n"));
        body_all.push_str(&format!("        ret.push(Box::new({}));\n", struct_name));
    }

    let str_impl_config = format!(
        r##"
impl Config {{
    pub fn new() -> Self {{
        toml::from_str("").unwrap()
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
}}"##,
        body, body_all
    );
    let _ = write!(out_impl_config, "{}", str_impl_config);

    // Output 'test.rs'
    let out_test = Path::new(&out_dir).join("test.rs");
    let mut out_test = File::create(&out_test).unwrap();

    let test_verbose = ["interface_port_with_modport"];

    for (file_name, _) in &rules {
        let silent = if test_verbose.contains(&file_name.as_str()) {
            "false"
        } else {
            "true"
        };
        let _ = write!(out_test, "#[test]\n");
        let _ = write!(out_test, "fn test_{}() {{\n", file_name);
        let _ = write!(out_test, "    test(\"{}\", {});\n", file_name, silent);
        let _ = write!(out_test, "}}\n");
    }
}
