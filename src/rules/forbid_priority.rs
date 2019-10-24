use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, UniquePriority};

pub struct ForbidPriority;

impl Rule for ForbidPriority {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::UniquePriority(UniquePriority::Priority(_)) => RuleResult::Fail(0),
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid priority")
    }

    fn hint(&self) -> String {
        String::from("'priority' is forbidden")
    }
}
