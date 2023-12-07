use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ProceduralContinuousAssignment;

impl SyntaxRule for ProceduralContinuousAssignment {
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
            RefNode::ProceduralContinuousAssignmentAssign(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("procedural_continuous_assignment")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Move continuous assignment out of procedural block.")
    }

    fn reason(&self) -> String {
        String::from("Continuous assigments ('assign a = b') in procedural blocks ('always*') are not synthesizable.")
    }
}
