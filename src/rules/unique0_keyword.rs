use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, UniquePriority};

pub struct Unique0Keyword;

impl Rule for Unique0Keyword {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::UniquePriority(UniquePriority::Unique0(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("unique0_keyword")
    }

    fn hint(&self) -> String {
        String::from("`unique0` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this causes mismatch between simulaton and synthesis")
    }
}
