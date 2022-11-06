use crate::linter::Rule;
use crate::rules::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub option: ConfigOption,
    #[serde(default)]
    pub rules: ConfigRules,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigOption {
    #[serde(with = "serde_regex", default)]
    pub exclude_paths: Vec<Regex>,

    #[serde(default = "default_indent")]
    pub indent: usize,
    #[serde(default = "default_prefix_inout")]
    pub prefix_inout: String,
    #[serde(default = "default_prefix_input")]
    pub prefix_input: String,
    #[serde(default = "default_prefix_output")]
    pub prefix_output: String,
    #[serde(default = "default_prefix_label")]
    pub prefix_label: String,
    #[serde(default = "default_prefix_instance")]
    pub prefix_instance: String,
    #[serde(default = "default_prefix_module")]
    pub prefix_module: String,
    #[serde(default = "default_prefix_package")]
    pub prefix_package: String,
    #[serde(default = "default_prefix_interface")]
    pub prefix_interface: String,

    #[serde(default = "default_re_lowercase")]
    pub re_required_checker: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_class: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_function: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_interface: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_modport: String,
    #[serde(default = "default_re_mixedcase")]
    pub re_required_module_ansi: String,
    #[serde(default = "default_re_uppercase")]
    pub re_required_module_nonansi: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_package: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_port_inout: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_port_input: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_port_output: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_port_ref: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_port_interface: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_program: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_task: String,

    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_checker: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_class: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_function: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_interface: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_modport: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_module_ansi: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_module_nonansi: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_package: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_port_inout: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_port_input: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_port_output: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_port_ref: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_port_interface: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_program: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_task: String,
}

include!(concat!(env!("OUT_DIR"), "/config_rules.rs"));

impl Default for ConfigOption {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

impl Default for ConfigRules {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

#[allow(dead_code)]
fn default_as_true() -> bool {
    true
}

#[allow(dead_code)]
fn default_as_false() -> bool {
    false
}

fn default_indent() -> usize {
    2
}

fn default_prefix_inout() -> String {
    String::from("b_")
}

fn default_prefix_input() -> String {
    String::from("i_")
}

fn default_prefix_output() -> String {
    String::from("o_")
}

fn default_prefix_label() -> String {
    String::from("l_")
}

fn default_prefix_instance() -> String {
    String::from("u_")
}

fn default_prefix_module() -> String {
    String::from("mod_")
}

fn default_prefix_package() -> String {
    String::from("pkg_")
}

fn default_prefix_interface() -> String {
    String::from("ifc_")
}

fn default_re_lowercase() -> String {
    String::from(r"^[a-z]+[a-z0-9_]*$")
}

fn default_re_mixedcase() -> String {
    String::from(r"^[a-z]+[a-zA-Z0-9_]*$")
}

fn default_re_uppercase() -> String {
    String::from(r"^[A-Z]+[A-Z0-9_]*$")
}

fn default_re_unconfigured() -> String {
    // Match all strings which don't begin with "X".
    // The "UNCONFIGURED" portion is an informative message, but functionally
    // redundant.
    // A special prefix "X" is required only for the testcases.
    String::from(r"^[^X](UNCONFIGURED|.*)$")
}

include!(concat!(env!("OUT_DIR"), "/impl_config.rs"));
