use crate::linter::{Rule, RuleResult};
use sv_parser::{AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LegacyAlways;

impl Rule for LegacyAlways {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
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
