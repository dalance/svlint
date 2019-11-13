use crate::config::{Config, ConfigOption};
use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};
use sv_parser::{unwrap_locate, Locate, RefNode, SyntaxTree};

#[derive(Clone, Copy)]
pub enum RuleResult {
    Pass,
    Fail,
    FailAt(usize, usize),
    FailLocate(Locate),
}

pub trait Rule {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult;
    fn name(&self) -> String;
    fn hint(&self) -> String;
    fn reason(&self) -> String;
}

pub struct Linter {
    option: ConfigOption,
    rules: Vec<Box<dyn Rule>>,
    plugins: Vec<Library>,
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
        Linter {
            option: config.option,
            rules,
            plugins: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &Path) {
        let lib = Library::new(path);
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

    pub fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> Vec<LintFailed> {
        let locate = if let Some(x) = unwrap_locate!(node.clone()) {
            x
        } else {
            return vec![];
        };

        let mut ret = Vec::new();
        'outer: for rule in &self.rules {
            match rule.check(&syntax_tree, &node) {
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
                            hint: rule.hint(),
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
                            hint: rule.hint(),
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
                            hint: rule.hint(),
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
