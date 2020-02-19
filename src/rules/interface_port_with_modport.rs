use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

pub struct InterfacePortWithModport;

impl Rule for InterfacePortWithModport {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
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
