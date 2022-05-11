use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixInstance;

impl Rule for PrefixInstance {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::NameOfInstance(x) => {
                let id: Option<&Locate> = match unwrap_node!(*x, InstanceIdentifier) {
                    Some(RefNode::InstanceIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                let is_prefixed: bool = match &id {
                    Some(x) => syntax_tree
                        .get_str(*x)
                        .unwrap()
                        .starts_with(&option.prefix_instance),
                    _ => false,
                };

                if is_prefixed {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_instance")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Module instance must have prefix \"{}\"",
            &option.prefix_instance
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }

    fn explanation(&self) -> String {
        String::from("TODO")
    }
}
