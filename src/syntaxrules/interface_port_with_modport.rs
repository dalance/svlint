use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct InterfacePortWithModport;

impl SyntaxRule for InterfacePortWithModport {
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
            RefNode::InterfacePortHeader(x) => {
                let a = unwrap_node!(*x, ModportIdentifier);
                if a.is_some() {
                    SyntaxRuleResult::Pass
                } else {
                    SyntaxRuleResult::Fail
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("interface_port_with_modport")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Specify the modport on the interface port.")
    }

    fn reason(&self) -> String {
        String::from("Without a modport, the interface port signals are all implictly `inout`.")
    }
}
