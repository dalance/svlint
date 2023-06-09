use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct CaseDefault;

impl SyntaxRule for CaseDefault {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::AlwaysConstruct(x) => {
                let (ref a, _) = x.nodes;
                match a {
                    AlwaysKeyword::AlwaysComb(_) => {
                        if let Some(x) = unwrap_node!(*x, CaseStatementNormal) {
                            let loc = unwrap_locate!(x.clone()).unwrap();
                            let a = unwrap_node!(x, CaseItemDefault);
                            if a.is_some() {
                                SyntaxRuleResult::Pass
                            } else {
                                SyntaxRuleResult::FailLocate(*loc)
                            }
                        } else {
                            SyntaxRuleResult::Pass
                        }
                    }
                    _ => SyntaxRuleResult::Pass,
                }
            }
            RefNode::FunctionDeclaration(x) => {
                if let Some(x) = unwrap_node!(*x, CaseStatementNormal) {
                    let loc = unwrap_locate!(x.clone()).unwrap();
                    let a = unwrap_node!(x, CaseItemDefault);
                    if a.is_some() {
                        SyntaxRuleResult::Pass
                    } else {
                        SyntaxRuleResult::FailLocate(*loc)
                    }
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("case_default")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use a `default` expression in `case` statements.")
    }

    fn reason(&self) -> String {
        String::from("Incomplete case may cause simulation/synthesis mismatch in `always_comb` and `function`.")
    }
}
