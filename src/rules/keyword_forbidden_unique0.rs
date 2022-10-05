use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree, UniquePriority};

#[derive(Default)]
pub struct KeywordForbiddenUnique0;

impl Rule for KeywordForbiddenUnique0 {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::UniquePriority(UniquePriority::Unique0(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_unique0")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`unique0` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this causes mismatch between simulation and synthesis")
    }
}
