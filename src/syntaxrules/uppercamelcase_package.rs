use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct UppercamelcasePackage;

impl Rule for UppercamelcasePackage {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::PackageIdentifier(x) => {
                let id: Option<&Locate> = match unwrap_node!(*x, SimpleIdentifier) {
                    Some(RefNode::SimpleIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                let is_uppercamelcase: bool = match &id {
                    Some(x) => syntax_tree
                        .get_str(*x)
                        .unwrap()
                        .chars()
                        .nth(0)
                        .unwrap()
                        .is_ascii_uppercase(),
                    _ => false,
                };

                if is_uppercamelcase {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("uppercamelcase_package")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!("Begin `package` name with UpperCamelCase."))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
