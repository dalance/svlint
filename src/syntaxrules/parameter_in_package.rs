use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ParameterInPackage {
    inside_param_port_list : bool,
}

impl SyntaxRule for ParameterInPackage {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(RefNode::ParameterPortList(_)) => {
                self.inside_param_port_list = true;
            },
            NodeEvent::Leave(RefNode::ParameterPortList(_)) => {
                self.inside_param_port_list = false;
            },
            NodeEvent::Enter(RefNode::ParameterDeclaration(&ref x)) => {
                if !self.inside_param_port_list {
                    let param_locate = unwrap_locate!(x).unwrap();
                    return SyntaxRuleResult::FailLocate(*param_locate)
                }
            },
            _ => {}
        };
        return SyntaxRuleResult::Pass
    }

    fn name(&self) -> String {
        String::from("parameter_in_package")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace `parameter` keyword with `localparam`.")
    }

    fn reason(&self) -> String {
        String::from("In a package, `localparam` properly describes the non-overridable semantics.")
    }
}
