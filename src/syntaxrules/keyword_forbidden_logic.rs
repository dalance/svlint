use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{IntegerVectorType, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct KeywordForbiddenLogic;

impl SyntaxRule for KeywordForbiddenLogic {
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
            RefNode::IntegerVectorType(IntegerVectorType::Logic(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_logic")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace `logic` keywords with `wire` or `reg`.")
    }

    fn reason(&self) -> String {
        String::from("Only SystemVerilog, not Verilog, has `logic`.")
    }
}
