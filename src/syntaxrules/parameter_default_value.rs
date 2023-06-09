use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ParameterDefaultValue {
    under_parameter_port_list: bool,
}

impl SyntaxRule for ParameterDefaultValue {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::ParameterPortList(_) => {
                        self.under_parameter_port_list = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::ParameterPortList(_) => {
                        self.under_parameter_port_list = false;
                    }
                    _ => ()
                }
                return SyntaxRuleResult::Pass;
            }
        };
        match (self.under_parameter_port_list, node) {
            (true, RefNode::ParamAssignment(x)) => {
                let (_, _, a) = &x.nodes;

                if a.is_none() {
                    SyntaxRuleResult::Fail
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("parameter_default_value")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Specify `parameter` with an explicit default value.")
    }

    fn reason(&self) -> String {
        String::from("Default values are required by some tools and clarify intent.")
    }
}
