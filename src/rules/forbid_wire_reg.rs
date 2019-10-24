use crate::linter::{Rule, RuleResult};
use sv_parser::{IntegerVectorType, NetType, RefNode, SyntaxTree};

pub struct ForbidWireReg;

impl Rule for ForbidWireReg {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::NetType(NetType::Wire(_)) => RuleResult::Fail(0),
            RefNode::IntegerVectorType(IntegerVectorType::Reg(_)) => RuleResult::Fail(0),
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid wire reg")
    }

    fn hint(&self) -> String {
        String::from("'wire'/'reg' must be replaced to 'logic'/'tri'")
    }
}
