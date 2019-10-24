use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, UniquePriority};

pub struct ForbidUnique;

impl Rule for ForbidUnique {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::UniquePriority(UniquePriority::Unique(_)) => RuleResult::Fail(0),
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid unique")
    }

    fn hint(&self) -> String {
        String::from("'unique' is forbidden")
    }
}
