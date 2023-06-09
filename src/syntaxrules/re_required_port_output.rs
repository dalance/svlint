use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree, PortDirection};

#[derive(Default)]
pub struct ReRequiredPortOutput {
    re: Option<Regex>,
    under_output_declaration: bool,
    under_ansi_port_declaration: bool,
    previous_port_direction_output: bool,
}

impl SyntaxRule for ReRequiredPortOutput {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_port_output).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::OutputDeclaration(_) => {
                        self.under_output_declaration = true;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = true;
                    }
                    RefNode::PortDirection(PortDirection::Output(_)) => {
                        self.previous_port_direction_output = true;
                    }
                    RefNode::ModuleAnsiHeader(_) |
                    RefNode::PortDirection(PortDirection::Inout(_)) |
                    RefNode::PortDirection(PortDirection::Input(_)) |
                    RefNode::PortDirection(PortDirection::Ref(_)) => {
                        self.previous_port_direction_output = false;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::OutputDeclaration(_) => {
                        self.under_output_declaration = false;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = false;
                    }
                    _ => ()
                }
                return SyntaxRuleResult::Pass;
            }
        };

        let c: bool = self.under_output_declaration ||
            (self.under_ansi_port_declaration && self.previous_port_direction_output);

        match (c, node) {
            (true, RefNode::PortIdentifier(x)) => {
                check_regex(true, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => SyntaxRuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_required_port_output")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a port identifier matching regex `{}`.",
            &option.re_required_port_output
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
