use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct KeywordForbiddenGenerate;

impl SyntaxRule for KeywordForbiddenGenerate {
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
            RefNode::GenerateRegion(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_generate")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove `generate`/`endgenerate` keywords.")
    }

    fn reason(&self) -> String {
        String::from("Keywords `generate`/`endgenerate` do not change semantics of generate blocks.")
    }
}
