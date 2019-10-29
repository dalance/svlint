use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NetType, PortDirection, RefNode, SyntaxTree};

pub struct InoutWithTri;

impl Rule for InoutWithTri {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(x.clone(), PortDirection);
                let is_inout = match dir {
                    Some(RefNode::PortDirection(PortDirection::Inout(_))) => true,
                    _ => false,
                };
                let net = unwrap_node!(x.clone(), NetType);
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
        String::from("inout with tri")
    }

    fn hint(&self) -> String {
        String::from("'inout' must have 'tri'")
    }
}
