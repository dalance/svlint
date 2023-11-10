use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct BlockingAssignmentInAlwaysAtEdge;

impl SyntaxRule for BlockingAssignmentInAlwaysAtEdge {
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
                let (t, x) = &x.nodes;
                match t {
                    AlwaysKeyword::Always(_) => {
                        let edge = unwrap_node!(x, EdgeIdentifier);
                        let blocking_assignment = unwrap_node!(x, BlockingAssignment);
                        let variable_assignment = unwrap_node!(x, VariableDeclAssignment);
                        if edge.is_some() && (blocking_assignment.is_some() || variable_assignment.is_some()) {
                            SyntaxRuleResult::Fail
                        } else {
                            SyntaxRuleResult::Pass
                        }
                    }
                    _ => SyntaxRuleResult::Pass,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("blocking_assignment_in_always_at_edge")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Do not use blocking assignments within edge-sensitive `always`.")
    }

    fn reason(&self) -> String {
        String::from("Blocking assignment in `always_ff` may cause undefined event ordering.")
    }
}
