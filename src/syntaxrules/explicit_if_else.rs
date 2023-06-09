use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitIfElse {
    under_always_construct: bool,
}

impl SyntaxRule for ExplicitIfElse {
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
            (true, RefNode::ConditionalStatement(x)) => {
                let (_, ref b, _, _, _, ref f) = &x.nodes;
                let loc = unwrap_locate!(b).unwrap();
                if f.is_none() {
                    SyntaxRuleResult::FailLocate(*loc)
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_if_else")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add an `else` clause to the `if` statement.")
    }

    fn reason(&self) -> String {
        String::from("Fully-specified conditional clarifies design intent.")
    }
}
