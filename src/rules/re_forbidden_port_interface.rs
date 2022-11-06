use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenPortInterface {
    re: Option<Regex>,
    under_list_of_interface_identifiers: bool,
    under_ansi_port_declaration: bool,
}

impl Rule for ReForbiddenPortInterface {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_port_interface).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::ListOfInterfaceIdentifiers(_) => {
                        self.under_list_of_interface_identifiers = true;
                    }
                    RefNode::InterfacePortHeader(_) => {
                        self.under_ansi_port_declaration = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::ListOfInterfaceIdentifiers(_) => {
                        self.under_list_of_interface_identifiers = false;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        match (self.under_list_of_interface_identifiers, self.under_ansi_port_declaration, node) {
            (true, _, RefNode::InterfaceIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            (_, true, RefNode::PortIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
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
        String::from("re_forbidden_port_interface")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a port identifier matching regex \"{}\".",
            &option.re_forbidden_port_interface
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
