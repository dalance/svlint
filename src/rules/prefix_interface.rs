use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixInterface {
    disable: bool,
}

impl Rule for PrefixInterface {
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
            RefNode::InterfaceIdentifier(x) => {
                let id: Option<&Locate> = match unwrap_node!(*x, SimpleIdentifier) {
                    Some(RefNode::SimpleIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                let is_prefixed: bool = match &id {
                    Some(x) => syntax_tree
                        .get_str(*x)
                        .unwrap()
                        .starts_with(&option.prefix_interface),
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
        String::from("prefix_interface")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "`interface` name must have prefix \"{}\"",
            &option.prefix_interface
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }

    fn disabled(&mut self, disable: Option<bool>) -> bool {
        match disable {
            Some(x) => {
                self.disable = x;
            }
            _ => {}
        }
        self.disable
    }
}
