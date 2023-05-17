use crate::config::ConfigOption;
use crate::linter::{check_prefix, SyntaxRule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixInstance;

impl SyntaxRule for PrefixInstance {
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
                check_prefix(unwrap_node!(*x, InstanceIdentifier), &syntax_tree, &option.prefix_instance)
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_instance")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Prefix instance identifier with \"{}\".",
            &option.prefix_instance
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention helps investigation using hierarchical paths.")
    }
}
