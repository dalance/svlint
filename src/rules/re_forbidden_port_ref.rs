use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree, PortDirection};

#[derive(Default)]
pub struct ReForbiddenPortRef {
    re: Option<Regex>,
    under_ref_declaration: bool,
    under_ansi_port_declaration: bool,
    previous_port_direction_ref: bool,
}

impl Rule for ReForbiddenPortRef {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_port_ref).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::RefDeclaration(_) => {
                        self.under_ref_declaration = true;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = true;
                    }
                    RefNode::PortDirection(PortDirection::Ref(_)) => {
                        self.previous_port_direction_ref = true;
                    }
                    RefNode::ModuleAnsiHeader(_) |
                    RefNode::PortDirection(PortDirection::Inout(_)) |
                    RefNode::PortDirection(PortDirection::Input(_)) |
                    RefNode::PortDirection(PortDirection::Output(_)) => {
                        self.previous_port_direction_ref = false;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::RefDeclaration(_) => {
                        self.under_ref_declaration = false;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        let c: bool = self.under_ref_declaration ||
            (self.under_ansi_port_declaration && self.previous_port_direction_ref);

        match (c, node) {
            (true, RefNode::PortIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_port_ref")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a port identifier matching regex \"{}\".",
            &option.re_forbidden_port_ref
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
