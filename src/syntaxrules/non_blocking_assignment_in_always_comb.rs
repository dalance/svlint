use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct NonBlockingAssignmentInAlwaysComb;

impl SyntaxRule for NonBlockingAssignmentInAlwaysComb {
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
                    AlwaysKeyword::AlwaysComb(_) => {
                        let nonblocking_assignment = unwrap_node!(x, NonblockingAssignment);
                        if nonblocking_assignment.is_some() {
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
        String::from("non_blocking_assignment_in_always_comb")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove non-blocking assignment in `always_comb`.")
    }

    fn reason(&self) -> String {
        String::from("Scheduling between blocking and non-blocking assignments is non-deterministic.")
    }
}
