use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{IntegerVectorType, NetType, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct KeywordForbiddenWireReg;

impl SyntaxRule for KeywordForbiddenWireReg {
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
            RefNode::NetType(NetType::Wire(_)) => SyntaxRuleResult::Fail,
            RefNode::IntegerVectorType(IntegerVectorType::Reg(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_forbidden_wire_reg")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace `wire` or `reg` keywords with `logic`, `tri` and/or `var`.")
    }

    fn reason(&self) -> String {
        String::from("Explicit datatype `logic` and/or datakind `var`/`tri` better describes intent.")
    }
}
