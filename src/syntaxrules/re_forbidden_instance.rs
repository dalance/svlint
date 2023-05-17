use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenInstance {
    re: Option<Regex>,
    under_hierarchical_instance: bool,
}

impl SyntaxRule for ReForbiddenInstance {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_instance).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::HierarchicalInstance(_) => {
                        self.under_hierarchical_instance = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::HierarchicalInstance(_) => {
                        self.under_hierarchical_instance = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        match (self.under_hierarchical_instance, node) {
            (true, RefNode::InstanceIdentifier(x)) => {
                check_regex(false, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_instance")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use an instance identifier not matching regex `{}`.",
            &option.re_forbidden_instance
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
