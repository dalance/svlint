use crate::config::ConfigOption;
use crate::linter::{check_prefix, Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixInput;

impl Rule for PrefixInput {
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
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_input: bool = match dir {
                    Some(RefNode::PortDirection(PortDirection::Input(_))) => true,
                    _ => false,
                };

                if is_input {
                    check_prefix(unwrap_node!(*x, PortIdentifier), &syntax_tree, &option.prefix_input)
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_input")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Prefix `input` port identifier with \"{}\".",
            &option.prefix_input
        ))
    }

    fn reason(&self) -> String {
        String::from("Port prefixes help readers to follow signals through modules.")
    }
}
