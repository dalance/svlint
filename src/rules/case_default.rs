use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, RefNode, SyntaxTree};

pub struct CaseDefault;

impl Rule for CaseDefault {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AlwaysConstruct(x) => {
                let (ref a, _) = x.nodes;
                match a {
                    AlwaysKeyword::AlwaysComb(_) => {
                        if let Some(x) = unwrap_node!(*x, CaseStatementNormal) {
                            let loc = unwrap_locate!(x.clone()).unwrap();
                            let a = unwrap_node!(x, CaseItemDefault);
                            if a.is_some() {
                                RuleResult::Pass
                            } else {
                                RuleResult::FailLocate(*loc)
                            }
                        } else {
                            RuleResult::Pass
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            RefNode::FunctionDeclaration(x) => {
                if let Some(x) = unwrap_node!(*x, CaseStatementNormal) {
                    let loc = unwrap_locate!(x.clone()).unwrap();
                    let a = unwrap_node!(x, CaseItemDefault);
                    if a.is_some() {
                        RuleResult::Pass
                    } else {
                        RuleResult::FailLocate(*loc)
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("case_default")
    }

    fn hint(&self) -> String {
        String::from("`case` must have `default` in `always_comb` or `function`")
    }

    fn reason(&self) -> String {
        String::from("'not full case' causes mismatch between simulation and synthesis")
    }
}
