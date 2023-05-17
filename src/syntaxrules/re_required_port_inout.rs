use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree, PortDirection};

#[derive(Default)]
pub struct ReRequiredPortInout {
    re: Option<Regex>,
    under_inout_declaration: bool,
    under_ansi_port_declaration: bool,
    previous_port_direction_inout: bool,
}

impl SyntaxRule for ReRequiredPortInout {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_port_inout).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::InoutDeclaration(_) => {
                        self.under_inout_declaration = true;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = true;
                    }
                    RefNode::ModuleAnsiHeader(_) |
                    RefNode::PortDirection(PortDirection::Inout(_)) => {
                        self.previous_port_direction_inout = true;
                    }
                    RefNode::PortDirection(PortDirection::Input(_)) |
                    RefNode::PortDirection(PortDirection::Output(_)) |
                    RefNode::PortDirection(PortDirection::Ref(_)) => {
                        self.previous_port_direction_inout = false;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::InoutDeclaration(_) => {
                        self.under_inout_declaration = false;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_declaration = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        let c: bool = self.under_inout_declaration ||
            (self.under_ansi_port_declaration && self.previous_port_direction_inout);

        match (c, node) {
            (true, RefNode::PortIdentifier(x)) => {
                check_regex(true, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_required_port_inout")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a port identifier matching regex `{}`.",
            &option.re_required_port_inout
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
