use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct NonBlockingAssignmentInAlwaysNoEdge;

impl SyntaxRule for NonBlockingAssignmentInAlwaysNoEdge {
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
                        let nonblocking_assignment = unwrap_node!(x, NonblockingAssignment);
                        if edge.is_none() && nonblocking_assignment.is_some() {
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
        String::from("non_blocking_assignment_in_always_no_edge")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove non-blocking assignment in combinational `always`.")
    }

    fn reason(&self) -> String {
        String::from("Scheduling between blocking and non-blocking assignments is non-deterministic.")
    }
}
