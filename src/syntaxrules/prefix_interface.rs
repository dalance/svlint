use crate::config::ConfigOption;
use crate::linter::{check_prefix, SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixInterface;

impl SyntaxRule for PrefixInterface {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::InterfaceDeclaration(x) => {
                check_prefix(unwrap_node!(*x, InterfaceIdentifier), &syntax_tree, &option.prefix_interface)
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_interface")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Prefix `interface` identifier with \"{}\".",
            &option.prefix_interface
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
