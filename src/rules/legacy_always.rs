use crate::linter::{Rule, RuleResult};
use sv_parser::{AlwaysKeyword, RefNode, SyntaxTree};

pub struct LegacyAlways;

impl Rule for LegacyAlways {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("legacy_always")
    }

    fn hint(&self) -> String {
        String::from("`always_comb`/`always_ff`/`always_latch` must be used")
    }

    fn reason(&self) -> String {
        String::from("`always` can't detect blocking/non-blocking mistake")
    }
}
