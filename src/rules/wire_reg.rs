use crate::linter::{Rule, RuleResult};
use sv_parser::{IntegerVectorType, NetType, NodeEvent, RefNode, SyntaxTree};

pub struct WireReg;

impl Rule for WireReg {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
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
