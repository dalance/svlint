use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct OperatorIncdec;

impl SyntaxRule for OperatorIncdec {
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
            RefNode::IncOrDecOperator(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("operator_incdec")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use `=` with a `+` or `-` instead of an increment or decrement operator.")
    }

    fn reason(&self) -> String {
        String::from("Only SystemVerilog, not Verilog, has increment and decrement operators.")
    }
}
