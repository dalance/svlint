use crate::config::{ConfigOption};
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct NonBlockingAssignmentInAlwaysComb;

impl Rule for NonBlockingAssignmentInAlwaysComb {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent,
             _option: &ConfigOption) -> RuleResult {
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
                    AlwaysKeyword::AlwaysComb(_) => {
                        let nonblocking_assignment = unwrap_node!(x, NonblockingAssignment);
                        if nonblocking_assignment.is_some() {
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
        String::from("non_blocking_assignment_in_always_comb")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("non-blocking assignment is forbidden in`always_comb`")
    }

    fn reason(&self) -> String {
        String::from("non-blocking assignment in `always_comb` causes elaboration error")
    }
}
