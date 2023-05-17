use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenLocalparam {
    re: Option<Regex>,
    under_local_parameter_declaration: bool,
}

impl SyntaxRule for ReForbiddenLocalparam {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_localparam).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::LocalParameterDeclaration(_) => {
                        self.under_local_parameter_declaration = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::LocalParameterDeclaration(_) => {
                        self.under_local_parameter_declaration = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        match (self.under_local_parameter_declaration, node) {
            (true, RefNode::ParameterIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_localparam")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a localparam identifier not matching regex `{}`.",
            &option.re_forbidden_localparam
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
