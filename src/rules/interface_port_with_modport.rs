use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub struct InterfacePortWithModport;

impl Rule for InterfacePortWithModport {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::InterfacePortHeader(x) => {
                let a = unwrap_node!(*x, ModportIdentifier);
                if a.is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("interface_port_with_modport")
    }

    fn hint(&self) -> String {
        String::from("interface port must have modport")
    }

    fn reason(&self) -> String {
        String::from("interface port without modport maybe `inout` at synthesis")
    }
}
