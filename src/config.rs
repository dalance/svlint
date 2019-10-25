use crate::linter::Rule;
use crate::rules::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub rules: ConfigRules,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigRules {
    #[serde(default = "default_as_true")]
    pub enum_with_type: bool,
    #[serde(default = "default_as_true")]
    pub generate_keyword: bool,
    #[serde(default = "default_as_true")]
    pub genvar_declaration: bool,
    #[serde(default = "default_as_true")]
    pub inout_with_tri: bool,
    #[serde(default = "default_as_true")]
    pub input_with_var: bool,
    #[serde(default = "default_as_true")]
    pub legacy_always: bool,
    #[serde(default = "default_as_true")]
    pub loop_variable_declaration: bool,
    #[serde(default = "default_as_true")]
    pub output_with_var: bool,
    #[serde(default = "default_as_true")]
    pub priority_keyword: bool,
    #[serde(default = "default_as_true")]
    pub tab_charactor: bool,
    #[serde(default = "default_as_true")]
    pub unique0_keyword: bool,
    #[serde(default = "default_as_true")]
    pub unique_keyword: bool,
    #[serde(default = "default_as_true")]
    pub wire_reg: bool,
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

impl Config {
    pub fn new() -> Self {
        toml::from_str("").unwrap()
    }

    pub fn gen_rules(&self) -> Vec<Box<dyn Rule>> {
        let mut ret: Vec<Box<dyn Rule>> = Vec::new();
        if self.rules.enum_with_type {
            ret.push(Box::new(EnumWithType));
        }
        if self.rules.generate_keyword {
            ret.push(Box::new(GenerateKeyword));
        }
        if self.rules.genvar_declaration {
            ret.push(Box::new(GenvarDeclaration));
        }
        if self.rules.inout_with_tri {
            ret.push(Box::new(InoutWithTri));
        }
        if self.rules.input_with_var {
            ret.push(Box::new(InputWithVar));
        }
        if self.rules.legacy_always {
            ret.push(Box::new(LegacyAlways));
        }
        if self.rules.loop_variable_declaration {
            ret.push(Box::new(LoopVariableDeclaration));
        }
        if self.rules.output_with_var {
            ret.push(Box::new(OutputWithVar));
        }
        if self.rules.priority_keyword {
            ret.push(Box::new(PriorityKeyword));
        }
        if self.rules.tab_charactor {
            ret.push(Box::new(TabCharactor));
        }
        if self.rules.unique0_keyword {
            ret.push(Box::new(Unique0Keyword));
        }
        if self.rules.unique_keyword {
            ret.push(Box::new(UniqueKeyword));
        }
        if self.rules.wire_reg {
            ret.push(Box::new(WireReg));
        }
        ret
    }
}
