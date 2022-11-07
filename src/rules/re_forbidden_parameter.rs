use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenParameter {
    re: Option<Regex>,
    under_parameter_declaration: bool,
    under_parameter_port_list: bool,
}

impl Rule for ReForbiddenParameter {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_parameter).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::ParameterDeclaration(_) => {
                        self.under_parameter_declaration = true;
                    }
                    RefNode::ParameterPortList(_) => {
                        self.under_parameter_port_list = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::ParameterDeclaration(_) => {
                        self.under_parameter_declaration = false;
                    }
                    RefNode::ParameterPortList(_) => {
                        self.under_parameter_port_list = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        let c: bool = self.under_parameter_declaration || self.under_parameter_port_list;

        match (c, node) {
            (true, RefNode::ParameterIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_parameter")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a parameter identifier not matching regex \"{}\".",
            &option.re_forbidden_parameter
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
