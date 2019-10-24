use crate::config::Config;
use std::path::PathBuf;
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

#[derive(Clone, Copy)]
pub enum RuleResult {
    Pass,
    Fail(usize),
}

pub trait Rule {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult;
    fn name(&self) -> String;
    fn hint(&self) -> String;
}

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

#[derive(Debug)]
pub struct LintFailed {
    pub path: PathBuf,
    pub beg: usize,
    pub len: usize,
    pub name: String,
    pub hint: String,
}

impl Linter {
    pub fn new(config: Config) -> Linter {
        let rules = config.gen_rules();
        Linter { rules }
    }

    pub fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> Vec<LintFailed> {
        let locate = unwrap_node!(node.clone(), Locate);
        let mut ret = Vec::new();
        for rule in &self.rules {
            if let RuleResult::Fail(offset) = rule.check(&syntax_tree, &node) {
                match locate {
                    Some(RefNode::Locate(x)) => {
                        if let Some((path, beg)) = syntax_tree.get_origin(&x) {
                            let beg = beg + offset;
                            let len = x.len;
                            let result = LintFailed {
                                path: path.clone(),
                                beg,
                                len,
                                name: rule.name(),
                                hint: rule.hint(),
                            };
                            ret.push(result);
                        }
                    }
                    _ => (),
                }
            }
        }
        ret
    }
}
