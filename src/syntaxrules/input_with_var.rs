use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct InputWithVar;

impl SyntaxRule for InputWithVar {
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
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_input = match dir {
                    Some(RefNode::PortDirection(PortDirection::Input(_))) => true,
                    _ => false,
                };
                let var = unwrap_node!(*x, VarDataTypeVar);
                if is_input && var.is_none() {
                    SyntaxRuleResult::Fail
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("input_with_var")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Specify `var` datakind on `input` ports.")
    }

    fn reason(&self) -> String {
        String::from("Default datakind of input port is a tri-state net.")

    }
}
