use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenVarClass {
    re: Option<Regex>,
    under_class_declaration: bool,
    under_class_method: bool,
    under_list_of_variable_decl_assignments: bool,
}

impl Rule for ReForbiddenVarClass {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_var_class).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::ClassDeclaration(_) => {
                        self.under_class_declaration = true;
                    }
                    RefNode::ClassMethod(_) => {
                        self.under_class_method = true;
                    }
                    RefNode::ListOfVariableDeclAssignments(_) => {
                        self.under_list_of_variable_decl_assignments = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::ClassDeclaration(_) => {
                        self.under_class_declaration = false;
                    }
                    RefNode::ClassMethod(_) => {
                        self.under_class_method = false;
                    }
                    RefNode::ListOfVariableDeclAssignments(_) => {
                        self.under_list_of_variable_decl_assignments = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        let c: bool = self.under_list_of_variable_decl_assignments &&
            self.under_class_declaration &&
            !self.under_class_method;

        match (c, node) {
            (true, RefNode::VariableIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_var_class")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a class-scoped variable identifier matching regex \"{}\".",
            &option.re_forbidden_var_class
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
