use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct OutputWithVar;

impl SyntaxRule for OutputWithVar {
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
                let is_output = match dir {
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => true,
                    _ => false,
                };
                let var = unwrap_node!(*x, VarDataTypeVar);
                if is_output && var.is_none() {
                    SyntaxRuleResult::Fail
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("output_with_var")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Specify `var` datakind on `output` ports.")
    }

    fn reason(&self) -> String {
        String::from("Explicit datakind of output ports should be consistent with input ports.")
    }
}
