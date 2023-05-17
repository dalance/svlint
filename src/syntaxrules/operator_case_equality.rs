use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, unwrap_locate, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct OperatorCaseEquality;

impl SyntaxRule for OperatorCaseEquality {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
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
            RefNode::BinaryOperator(x) => {
                let loc: Option<&Locate> = match unwrap_node!(*x, Symbol) {
                    Some(RefNode::Symbol(symbol_)) => {
                        unwrap_locate!(symbol_)
                    }
                    _ => None,
                };

                if let Some(loc) = loc {
                    let s = syntax_tree.get_str(loc).unwrap();

                    if (s == "===") || (s == "!==") {
                        SyntaxRuleResult::Fail
                    } else {
                        SyntaxRuleResult::Pass
                    }
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("operator_case_equality")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use logical equality instead of case equality.")
    }

    fn reason(&self) -> String {
        String::from("Case equality operations are not generally synthesizable.")
    }
}
