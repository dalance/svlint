use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitCaseDefault {
    under_always_construct: bool,
}

impl SyntaxRule for ExplicitCaseDefault {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::AlwaysConstruct(_) => {
                        self.under_always_construct = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::AlwaysConstruct(_) => {
                        self.under_always_construct = false;
                    }
                    _ => ()
                }
                return SyntaxRuleResult::Pass;
            }
        };
        match (self.under_always_construct, node) {
            (true, RefNode::CaseStatementNormal(x)) => {
                let a = unwrap_node!(*x, CaseItemDefault);
                if a.is_some() {
                    SyntaxRuleResult::Pass
                } else {
                    SyntaxRuleResult::Fail
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_case_default")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add a `default` arm to the `case` statement.")
    }

    fn reason(&self) -> String {
        String::from("Fully-specified case clarifies design intent.")
    }
}
