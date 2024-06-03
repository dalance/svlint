use crate::linter::{SyntaxRule, TextRule};
use crate::rules::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub option: ConfigOption,

    #[serde(default)]
    pub textrules: ConfigTextRules,

    // Pre-v0.7.2, svlint supports only syntaxrules, so they're just called
    // "rules" (instead of "syntaxrules").
    // The serde alias allows either "rules" or "syntaxrules" to be used in
    // the configuration files, usually `.svlint.toml`.
    #[serde(default, alias = "rules")]
    pub syntaxrules: ConfigSyntaxRules,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigOption {
    #[serde(with = "serde_regex", default)]
    pub exclude_paths: Vec<Regex>,

    #[serde(default = "default_textwidth")]
    pub textwidth: usize,

    #[serde(default = "default_copyright_linenum")]
    pub copyright_linenum: usize,
    #[serde(default = "default_copyright_year")]
    pub copyright_year: String,
    #[serde(default = "default_copyright_holder")]
    pub copyright_holder: String,

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
    pub re_required_assert: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_assert_property: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_checker: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_class: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_function: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_generateblock: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_genvar: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_instance: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_interface: String,
    #[serde(default = "default_re_uppercase")]
    pub re_required_localparam: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_modport: String,
    #[serde(default = "default_re_mixedcase")]
    pub re_required_module_ansi: String,
    #[serde(default = "default_re_uppercase")]
    pub re_required_module_nonansi: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_package: String,
    #[serde(default = "default_re_uppercase")]
    pub re_required_parameter: String,
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
    pub re_required_property: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_sequence: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_task: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_var_class: String,
    #[serde(default = "default_re_lowercase")]
    pub re_required_var_classmethod: String,

    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_assert: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_assert_property: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_checker: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_class: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_function: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_generateblock: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_genvar: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_instance: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_interface: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_localparam: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_modport: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_module_ansi: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_module_nonansi: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_package: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_parameter: String,
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
    pub re_forbidden_property: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_sequence: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_task: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_var_class: String,
    #[serde(default = "default_re_unconfigured")]
    pub re_forbidden_var_classmethod: String,

    #[serde(default)]
    pub unpacked_array: UnpackedArrayOption,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnpackedArrayOption {
    #[serde(default)]
    pub localparam_declaration: bool,
    #[serde(default)]
    pub param_declaration: bool,
    #[serde(default)]
    pub specparam_declaration: bool,
    #[serde(default)]
    pub inout_declaration: bool,
    #[serde(default)]
    pub ansi_port_declaration: bool,
    #[serde(default)]
    pub input_declaration: bool,
    #[serde(default)]
    pub output_declaration: bool,
    #[serde(default)]
    pub interface_port_declaration: bool,
    #[serde(default)]
    pub ref_declaration: bool,
    #[serde(default)]
    pub data_declaration: bool,
    #[serde(default)]
    pub net_declaration: bool,
}

include!(concat!(env!("OUT_DIR"), "/config_rules.rs"));

impl Default for ConfigOption {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

impl Default for ConfigTextRules {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

impl Default for ConfigSyntaxRules {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

impl Default for UnpackedArrayOption {
    fn default() -> Self {
        Self {
            localparam_declaration: false,
            param_declaration: false,
            specparam_declaration: false,
            inout_declaration: false,
            ansi_port_declaration: false,
            input_declaration: false,
            output_declaration: false,
            interface_port_declaration: false,
            ref_declaration: false,
            data_declaration: true,
            net_declaration: false,
        }
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

fn default_textwidth() -> usize {
    80
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

fn default_copyright_linenum() -> usize {
    1
}

fn default_copyright_year() -> String {
    String::from("1234")
}

fn default_copyright_holder() -> String {
    String::from(r"HOLDER")
}

include!(concat!(env!("OUT_DIR"), "/impl_config.rs"));
