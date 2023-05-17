use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenClass {
    re: Option<Regex>,
}

impl SyntaxRule for ReForbiddenClass {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_class).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::ClassDeclaration(x) => {
                check_regex(false, unwrap_node!(*x, ClassIdentifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_class")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a class identifier not matching regex `{}`.",
            &option.re_forbidden_class
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
