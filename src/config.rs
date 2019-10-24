use crate::linter::Rule;
use crate::rules::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_as_true")]
    pub forbid_always: bool,
    #[serde(default = "default_as_true")]
    pub forbid_generate: bool,
    #[serde(default = "default_as_true")]
    pub forbid_priority: bool,
    #[serde(default = "default_as_true")]
    pub forbid_tab: bool,
    #[serde(default = "default_as_true")]
    pub forbid_unique: bool,
    #[serde(default = "default_as_true")]
    pub forbid_unique0: bool,
    #[serde(default = "default_as_true")]
    pub forbid_wire_reg: bool,
}

#[allow(dead_code)]
fn default_as_true() -> bool {
    true
}

#[allow(dead_code)]
fn default_as_false() -> bool {
    false
}

impl Config {
    pub fn new() -> Self {
        toml::from_str("").unwrap()
    }

    pub fn gen_rules(&self) -> Vec<Box<dyn Rule>> {
        let mut ret: Vec<Box<dyn Rule>> = Vec::new();
        if self.forbid_always {
            ret.push(Box::new(ForbidAlways));
        }
        if self.forbid_generate {
            ret.push(Box::new(ForbidGenerate));
        }
        if self.forbid_priority {
            ret.push(Box::new(ForbidPriority));
        }
        if self.forbid_tab {
            ret.push(Box::new(ForbidTab));
        }
        if self.forbid_unique {
            ret.push(Box::new(ForbidUnique));
        }
        if self.forbid_unique0 {
            ret.push(Box::new(ForbidUnique0));
        }
        if self.forbid_wire_reg {
            ret.push(Box::new(ForbidWireReg));
        }
        ret
    }
}
