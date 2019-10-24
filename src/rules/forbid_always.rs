use crate::linter::{Rule, RuleResult};
use sv_parser::{AlwaysKeyword, RefNode, SyntaxTree};

pub struct ForbidAlways;

impl Rule for ForbidAlways {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => RuleResult::Fail(0),
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid always")
    }

    fn hint(&self) -> String {
        String::from("'always_comb'/'always_ff'/'always_latch' must be used")
    }
}
