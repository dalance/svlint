use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReRequiredGenvar {
    re: Option<Regex>,
    under_genvar_declaration: bool,
    under_genvar_initialization: bool,
}

impl Rule for ReRequiredGenvar {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_genvar).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::GenvarDeclaration(_) => {
                        self.under_genvar_declaration = true;
                    }
                    RefNode::GenvarInitialization(_) => {
                        self.under_genvar_initialization = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::GenvarDeclaration(_) => {
                        self.under_genvar_declaration = false;
                    }
                    RefNode::GenvarInitialization(_) => {
                        self.under_genvar_initialization = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        let c: bool = self.under_genvar_declaration ||
            self.under_genvar_initialization;

        match (c, node) {
            (true, RefNode::GenvarIdentifier(x)) => {
                check_regex(true, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_required_genvar")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a genvar identifier matching regex \"{}\".",
            &option.re_required_genvar
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
