use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LowercamelcaseModule;

impl SyntaxRule for LowercamelcaseModule {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::ModuleIdentifier(x) => {
                let id: Option<&Locate> = match unwrap_node!(*x, SimpleIdentifier) {
                    Some(RefNode::SimpleIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                let is_lowercamelcase: bool = match &id {
                    Some(x) => syntax_tree
                        .get_str(*x)
                        .unwrap()
                        .chars()
                        .nth(0)
                        .unwrap()
                        .is_ascii_lowercase(),
                    _ => false,
                };

                if is_lowercamelcase {
                    SyntaxRuleResult::Pass
                } else {
                    SyntaxRuleResult::Fail
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("lowercamelcase_module")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!("Begin `module` name with lowerCamelCase."))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
