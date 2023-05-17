use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct KeywordForbiddenAlways;

impl SyntaxRule for KeywordForbiddenAlways {
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
            RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_always")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use `always_comb`/`always_ff`/`always_latch` instead of `always`.")
    }

    fn reason(&self) -> String {
        String::from("General-purpose `always` cannot detect combinatorial/stateful (non-)blocking mistakes.")
    }
}
