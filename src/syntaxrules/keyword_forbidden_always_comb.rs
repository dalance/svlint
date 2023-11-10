use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct KeywordForbiddenAlwaysComb;

impl SyntaxRule for KeywordForbiddenAlwaysComb {
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
            RefNode::AlwaysKeyword(AlwaysKeyword::AlwaysComb(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_always_comb")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use `always @*` instead of `always_comb`.")
    }

    fn reason(&self) -> String {
        String::from("Only SystemVerilog, not Verilog, has `always_comb`.")
    }
}
