use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree, UniquePriority};

#[derive(Default)]
pub struct KeywordForbiddenUnique;

impl SyntaxRule for KeywordForbiddenUnique {
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
            RefNode::UniquePriority(UniquePriority::Unique(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_unique")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove `unique` keyword, perhaps replace with an assertion.")
    }

    fn reason(&self) -> String {
        String::from("Unique-case/if constructs may mismatch between simulation and synthesis.")
    }
}
