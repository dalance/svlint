use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitCaseDefault {
    under_always_construct: bool,
}

impl Rule for ExplicitCaseDefault {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
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
                return RuleResult::Pass;
            }
        };
        match (self.under_always_construct, node) {
            (true, RefNode::CaseStatementNormal(x)) => {
                let a = unwrap_node!(*x, CaseItemDefault);
                if a.is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
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
