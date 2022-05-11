use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitIfElse;

impl Rule for ExplicitIfElse {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::AlwaysConstruct(x) => {
                if let Some(x) = unwrap_node!(*x, ConditionalStatement) {
                    if let RefNode::ConditionalStatement(y) = x {
                        let (_, ref b, _, _, _, ref f) = &y.nodes;
                        let loc = unwrap_locate!(b).unwrap();
                        if f.is_none() {
                            RuleResult::FailLocate(*loc)
                        } else {
                            RuleResult::Pass
                        }
                    } else {
                        RuleResult::Pass
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_if_else")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`if` must have `else` in `always*`")
    }

    fn reason(&self) -> String {
        String::from("explicit `else` makes design intent clearer")
    }

    fn explanation(&self) -> String {
        String::from("TODO")
    }
}
