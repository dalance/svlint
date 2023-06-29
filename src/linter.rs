use crate::config::{Config, ConfigOption};
use libloading::{Library, Symbol};
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sv_parser::{unwrap_locate, Locate, NodeEvent, RefNode, SyntaxTree};

// Rule enum is for use by plugins.
#[derive(Clone, Copy)]
pub enum Rule {
    Text(*mut dyn TextRule),
    Syntax(*mut dyn SyntaxRule),
}

// Macro for use within plugins.
// Example usage in a plugin's `get_plugin` function:
//    pluginrules!(
//        SamplePlugin,
//        AnotherPlugin,
//    )
#[macro_export]
macro_rules! pluginrules {
    ( $( $x:ty ),* $(,)? ) => {
        {
            let mut vec: Vec<Rule> = Vec::new();
            $(
                let rule = <$x>::default();
                vec.push(rule.into_rule());
            )*
            vec
        }
    };
}

#[derive(Clone, Copy)]
pub enum TextRuleResult {
    Pass,
    Fail {
        offset: usize, // Character index, on this line, beginning failure.
        len: usize, // Number of characters causing failure.
    },
}

pub trait TextRule: Sync + Send {
    fn check(
        &mut self,
        line: Option<&str>,
        config: &ConfigOption,
    ) -> TextRuleResult;
    fn name(&self) -> String;
    fn hint(&self, config: &ConfigOption) -> String;
    fn reason(&self) -> String;

    fn into_rule(self) -> Rule
    where
        Self: Sized + 'static,
    {
        let temp = Box::new(self);
        Rule::Text(Box::into_raw(temp))
    }
}

#[derive(Clone, Copy)]
pub enum SyntaxRuleResult {
    Pass,
    Fail,
    FailAt(usize, usize),
    FailLocate(Locate),
}

pub trait SyntaxRule: Sync + Send {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        config: &ConfigOption,
    ) -> SyntaxRuleResult;
    fn name(&self) -> String;
    fn hint(&self, config: &ConfigOption) -> String;
    fn reason(&self) -> String;

    fn into_rule(self) -> Rule
    where
        Self: Sized + 'static,
    {
        let temp = Box::new(self);
        Rule::Syntax(Box::into_raw(temp))
    }
}

pub struct Linter {
    option: ConfigOption,
    textrules: Vec<Box<dyn TextRule>>,
    syntaxrules: Vec<Box<dyn SyntaxRule>>,
    plugins: Vec<Library>,
    re_ctl: Regex,
    pub ctl_enabled: HashMap<String, bool>,
}

#[derive(Debug)]
pub struct LintFailed {
    pub path: PathBuf,
    pub beg: usize,
    pub len: usize,
    pub name: String,
    pub hint: String,
    pub reason: String,
}

impl Linter {
    pub fn new(config: Config) -> Linter {
        let textrules = config.gen_textrules();
        let syntaxrules = config.gen_syntaxrules();

        // NOTE: Only syntaxrules are comment-controllable, not textrules.
        let re_ctl = Regex::new(r"/\*\s*svlint\s+(on|off)\s+([a-z0-9_]+)\s*\*/").unwrap();
        let mut ctl_enabled = HashMap::new();
        for rule in &syntaxrules {
            ctl_enabled.insert(rule.name(), true);
        }

        Linter {
            option: config.option,
            textrules,
            syntaxrules,
            plugins: Vec::new(),
            re_ctl,
            ctl_enabled,
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<(), libloading::Error> {
        let lib = unsafe { Library::new(path) }?;

        self.plugins.push(lib);
        let lib = self.plugins.last().unwrap();

        let get_plugin: Result<Symbol<extern "C" fn() -> Vec<Rule>>, _> =
            unsafe { lib.get(b"get_plugin") };
        if let Ok(get_plugin) = get_plugin {
            let plugins = get_plugin();
            for plugin in plugins {
                match plugin {
                    Rule::Text(p) => {
                        let plugin = unsafe { Box::from_raw(p) };
                        self.textrules.push(plugin);
                    },
                    Rule::Syntax(p) => {
                        let plugin = unsafe { Box::from_raw(p) };
                        self.ctl_enabled.insert(plugin.name(), true);
                        self.syntaxrules.push(plugin);
                    },
                }
            }
        }

        Ok(())
    }

    pub fn textrules_check(&mut self, line: Option<&str>, path: &Path, beg: &usize) -> Vec<LintFailed> {

        let mut ret = Vec::new();
        'outer: for rule in &mut self.textrules {
            match rule.check(line, &self.option) {
                TextRuleResult::Fail {offset, len} => {
                    if line.is_some() {
                        for exclude in &self.option.exclude_paths {
                            if exclude.is_match(&path.to_string_lossy()) {
                                continue 'outer;
                            }
                        }
                        let result = LintFailed {
                            path: path.to_path_buf(),
                            beg: beg + offset,
                            len,
                            name: rule.name(),
                            hint: rule.hint(&self.option),
                            reason: rule.reason(),
                        };
                        ret.push(result);
                    }
                }
                _ => (),
            }
        }
        ret
    }

    fn update_ctl_enabled(&mut self, syntax_tree: &SyntaxTree, event: &NodeEvent) {
        match event {
            NodeEvent::Enter(RefNode::Comment(x)) => {
                let loc: Option<&Locate> = unwrap_locate!(*x);
                let text: Option<&str> = match &loc {
                    Some(x) => syntax_tree.get_str(*x),
                    _ => None,
                };
                let caps = self.re_ctl.captures(text.unwrap());
                if caps.is_some() {
                    let caps = caps.unwrap();
                    let ctl_name = caps.get(2).unwrap().as_str();
                    if self.ctl_enabled.contains_key(ctl_name) {
                        let ctl_enable = match caps.get(1).unwrap().as_str() {
                            "off" => false,
                            _ => true,
                        };
                        self.ctl_enabled.insert(ctl_name.to_string(), ctl_enable);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn syntaxrules_check(&mut self, syntax_tree: &SyntaxTree, event: &NodeEvent) -> Vec<LintFailed> {
        self.update_ctl_enabled(syntax_tree, event);

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(x) => x,
        };
        let locate = if let Some(x) = unwrap_locate!(node.clone()) {
            x
        } else {
            return vec![];
        };

        let mut ret = Vec::new();
        'outer: for rule in &mut self.syntaxrules {
            match self.ctl_enabled[&rule.name()] {
                true => {}
                _ => {
                    continue 'outer;
                }
            }

            match rule.check(syntax_tree, event, &self.option) {
                SyntaxRuleResult::Fail => {
                    if let Some((path, beg)) = syntax_tree.get_origin(&locate) {
                        for exclude in &self.option.exclude_paths {
                            if exclude.is_match(&path.to_string_lossy()) {
                                continue 'outer;
                            }
                        }
                        let result = LintFailed {
                            path: path.clone(),
                            beg,
                            len: locate.len,
                            name: rule.name(),
                            hint: rule.hint(&self.option),
                            reason: rule.reason(),
                        };
                        ret.push(result);
                    }
                }
                SyntaxRuleResult::FailAt(offset, len) => {
                    if let Some((path, beg)) = syntax_tree.get_origin(&locate) {
                        for exclude in &self.option.exclude_paths {
                            if exclude.is_match(&path.to_string_lossy()) {
                                continue 'outer;
                            }
                        }
                        let result = LintFailed {
                            path: path.clone(),
                            beg: beg + offset,
                            len,
                            name: rule.name(),
                            hint: rule.hint(&self.option),
                            reason: rule.reason(),
                        };
                        ret.push(result);
                    }
                }
                SyntaxRuleResult::FailLocate(x) => {
                    if let Some((path, beg)) = syntax_tree.get_origin(&x) {
                        for exclude in &self.option.exclude_paths {
                            if exclude.is_match(&path.to_string_lossy()) {
                                continue 'outer;
                            }
                        }
                        let result = LintFailed {
                            path: path.clone(),
                            beg,
                            len: x.len,
                            name: rule.name(),
                            hint: rule.hint(&self.option),
                            reason: rule.reason(),
                        };
                        ret.push(result);
                    }
                }
                _ => (),
            }
        }
        ret
    }
}

// Utility function used by syntaxrules `re_(required|forbidden)_*`.
pub fn check_regex(
    required_not_forbidden: bool,
    id: Option<RefNode>,
    syntax_tree: &SyntaxTree,
    re: &Regex,
) -> SyntaxRuleResult {
    let loc: &Locate = match id {
        Some(x) => unwrap_locate!(x),
        _ => None,
    }
    .unwrap();

    let is_match: bool = re.is_match(syntax_tree.get_str(loc).unwrap());

    if is_match == required_not_forbidden {
        SyntaxRuleResult::Pass
    } else {
        SyntaxRuleResult::Fail
    }
}

// Utility function used by syntaxrules `prefix_*`.
pub fn check_prefix(
    id: Option<RefNode>,
    syntax_tree: &SyntaxTree,
    prefix: &String,
) -> SyntaxRuleResult {
    let loc: &Locate = match id {
        Some(x) => unwrap_locate!(x),
        _ => None,
    }
    .unwrap();

    let is_prefixed: bool = syntax_tree.get_str(loc).unwrap().starts_with(prefix);

    if is_prefixed {
        SyntaxRuleResult::Pass
    } else {
        SyntaxRuleResult::Fail
    }
}
