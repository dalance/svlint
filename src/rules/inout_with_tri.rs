use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NetType, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct InoutWithTri;

impl Rule for InoutWithTri {
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
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_inout = match dir {
                    Some(RefNode::PortDirection(PortDirection::Inout(_))) => true,
                    _ => false,
                };
                let net = unwrap_node!(*x, NetType);
                let is_tri = match net {
                    Some(RefNode::NetType(NetType::Tri(_))) => true,
                    _ => false,
                };
                if is_inout && !is_tri {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("inout_with_tri")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`inout` must have `tri`")
    }

    fn reason(&self) -> String {
        String::from("")
    }
}
