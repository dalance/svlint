use crate::linter::{Rule, RuleResult};
use sv_parser::{IntegerVectorType, NetType, RefNode, SyntaxTree};

pub struct WireReg;

impl Rule for WireReg {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::NetType(NetType::Wire(_)) => RuleResult::Fail,
            RefNode::IntegerVectorType(IntegerVectorType::Reg(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("wire_reg")
    }

    fn hint(&self) -> String {
        String::from("`wire`/`reg` must be replaced to `logic`/`tri`")
    }

    fn reason(&self) -> String {
        String::from("`logic` can detect multi-drive")
    }
}
