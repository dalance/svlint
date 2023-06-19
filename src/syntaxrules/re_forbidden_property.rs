use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenProperty {
    re: Option<Regex>,
}

impl SyntaxRule for ReForbiddenProperty {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_property).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };

        match node {
            RefNode::PropertyDeclaration(x) => {
                check_regex(false, unwrap_node!(*x, PropertyIdentifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_property")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a property identifier not matching regex `{}`.",
            &option.re_forbidden_property
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
