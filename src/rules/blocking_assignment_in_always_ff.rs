use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct BlockingAssignmentInAlwaysFf;

impl Rule for BlockingAssignmentInAlwaysFf {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::AlwaysConstruct(x) => {
                let (t, x) = &x.nodes;
                match t {
                    AlwaysKeyword::AlwaysFf(_) => {
                        let blocking_assignment = unwrap_node!(x, BlockingAssignment);
                        let variable_assignment = unwrap_node!(x, VariableDeclAssignment);
                        if blocking_assignment.is_some() || variable_assignment.is_some() {
                            RuleResult::Fail
                        } else {
                            RuleResult::Pass
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("blocking_assignment_in_always_ff")
    }

    fn hint(&self) -> String {
        String::from("blocking assignment is forbidden in`always_ff`")
    }

    fn reason(&self) -> String {
        String::from("blocking assignment in `always_ff` causes elaboration error")
    }
}
