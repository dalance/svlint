use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReRequiredLocalparam {
    re: Option<Regex>,
    under_local_parameter_declaration: bool,
}

impl SyntaxRule for ReRequiredLocalparam {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_localparam).unwrap());
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
                return SyntaxRuleResult::Pass;
            }
        };

        match (self.under_local_parameter_declaration, node) {
            (true, RefNode::ParameterIdentifier(x)) => {
                check_regex(true, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => SyntaxRuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_required_localparam")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a localparam identifier matching regex `{}`.",
            &option.re_required_localparam
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
