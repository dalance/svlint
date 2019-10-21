use crate::rules::*;
use std::path::PathBuf;
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub trait Rule {
    fn check(&self, node: &RefNode) -> bool;
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
    pub fn new() -> Linter {
        let rules = vec![Box::new(OmitGenerate) as Box<dyn Rule>];
        Linter { rules }
    }

    pub fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> Vec<LintFailed> {
        let locate = unwrap_node!(node.clone(), Locate).unwrap();
        let mut ret = Vec::new();
        for rule in &self.rules {
            let pass = rule.check(&node);
            if !pass {
                match locate {
                    RefNode::Locate(x) => {
                        if let Some((path, beg)) = syntax_tree.get_origin(&x) {
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
