use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct BlockingAssignmentInAlwaysFf;

impl Rule for BlockingAssignmentInAlwaysFf {
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

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Do not use blocking assignments within `always_ff`.")
    }

    fn reason(&self) -> String {
        String::from("Blocking assignment in `always_ff` may cause undefined event ordering.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        Simulator event ordering between blocking and non-blocking assignments
        is undefined, so observed behavior simulator-dependent.
        As all examples in IEEE1800-2017 show, `always_ff` should only contain
        non-blocking assignments in order for sampling and variable evaluation
        to operate in a defined order.

        Specifically, `always_ff` constructs should not contain blocking assignments:
          - Blocking assignment operator, e.g. `foo = 123;`
          - Increment/decrement operators, e.g. `foo++;`, `foo--;`.

        The most relevant clauses of IEEE1800-2017 are:
          - 9.2.2.4 Sequential logic always_ff procedure
          - 9.4.2 Event control
          - 10.4.1 Blocking procedural assignments
          - 10.4.2 Nonblocking procedural assignments
          - 16.5.1 Sampling
        "})
    }
}
