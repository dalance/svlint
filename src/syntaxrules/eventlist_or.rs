use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct EventlistOr;

impl SyntaxRule for EventlistOr {
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
            RefNode::EventExpressionOr(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("eventlist_or")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use comma event expression separator instead of `or`.")
    }

    fn reason(&self) -> String {
        String::from("Consistent separators enhance readability.")
    }
}
