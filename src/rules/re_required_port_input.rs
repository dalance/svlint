use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree, PortDirection};

#[derive(Default)]
pub struct ReRequiredPortInput {
    re: Option<Regex>,
    under_input_declaration: bool,
    under_ansi_port_declaration: bool,
    previous_port_direction_input: bool,
}

impl Rule for ReRequiredPortInput {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_port_input).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::InputDeclaration(_) => {
                        self.under_input_declaration = true;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = true;
                    }
                    RefNode::PortDirection(PortDirection::Input(_)) => {
                        self.previous_port_direction_input = true;
                    }
                    RefNode::ModuleAnsiHeader(_) |
                    RefNode::PortDirection(PortDirection::Inout(_)) |
                    RefNode::PortDirection(PortDirection::Output(_)) |
                    RefNode::PortDirection(PortDirection::Ref(_)) => {
                        self.previous_port_direction_input = false;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::InputDeclaration(_) => {
                        self.under_input_declaration = false;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        let c: bool = self.under_input_declaration ||
            (self.under_ansi_port_declaration && self.previous_port_direction_input);

        match (c, node) {
            (true, RefNode::PortIdentifier(x)) => {
                check_regex(true, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => {
                println!("{:?}", node);
                println!();
                RuleResult::Pass
            }
        }
    }

    fn name(&self) -> String {
        String::from("re_required_port_input")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a port identifier matching regex \"{}\".",
            &option.re_required_port_input
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
