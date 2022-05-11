use crate::config::{Config, ConfigOption};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sv_parser::{unwrap_locate, Locate, NodeEvent, SyntaxTree};

#[derive(Clone, Copy)]
pub enum RuleResult {
    Pass,
    Fail,
    FailAt(usize, usize),
    FailLocate(Locate),
}

pub trait Rule: Sync + Send {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        config: &ConfigOption,
    ) -> RuleResult;
    fn name(&self) -> String;
    fn hint(&self, config: &ConfigOption) -> String;
    fn reason(&self) -> String;
    fn explanation(&self) -> String;
}

pub struct Linter {
    option: ConfigOption,
    rules: Vec<Box<dyn Rule>>,
    plugins: Vec<Library>,
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
        let rules = config.gen_rules();

        let mut ctl_enabled = HashMap::new();
        for rule in &rules {
            ctl_enabled.insert(rule.name(), true);
        }

        Linter {
            option: config.option,
            rules,
            plugins: Vec::new(),
            ctl_enabled,
        }
    }

    pub fn load(&mut self, path: &Path) {
        let lib = unsafe { Library::new(path) };
        if let Ok(lib) = lib {
            self.plugins.push(lib);
            let lib = self.plugins.last().unwrap();

            let get_plugin: Result<Symbol<extern "C" fn() -> *mut dyn Rule>, _> =
                unsafe { lib.get(b"get_plugin") };
            if let Ok(get_plugin) = get_plugin {
                let plugin = unsafe { Box::from_raw(get_plugin()) };
                self.rules.push(plugin);
            }
        }
    }

    pub fn check(&mut self, syntax_tree: &SyntaxTree, event: &NodeEvent) -> Vec<LintFailed> {
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
        'outer: for rule in &mut self.rules {
            match self.ctl_enabled[&rule.name()] {
                true => {}
                _ => { continue 'outer; }
            }

            match rule.check(syntax_tree, event, &self.option) {
                RuleResult::Fail => {
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
                RuleResult::FailAt(offset, len) => {
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
                RuleResult::FailLocate(x) => {
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
