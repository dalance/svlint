use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, UniquePriority};

pub struct ForbidUnique0;

impl Rule for ForbidUnique0 {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::UniquePriority(UniquePriority::Unique0(_)) => RuleResult::Fail(0),
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid unique0")
    }

    fn hint(&self) -> String {
        String::from("'unique0' is forbidden")
    }
}
