use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{
    unwrap_locate, unwrap_node, Locate, NodeEvent, PortDirection, RefNode, SyntaxTree,
};

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

                let id: Option<&Locate> = match unwrap_node!(*x, PortIdentifier) {
                    Some(RefNode::PortIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                let is_prefixed: bool = match &id {
                    Some(x) => syntax_tree
                        .get_str(*x)
                        .unwrap()
                        .starts_with(&option.prefix_input),
                    _ => false,
                };

                match (is_input, is_prefixed) {
                    (true, false) => RuleResult::Fail,
                    _ => RuleResult::Pass,
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
            "Prefix `input` identifier with \"{}\".",
            &option.prefix_input
        ))
    }

    fn reason(&self) -> String {
        String::from("Port prefixes help readers to follow signals through modules.")
    }
}
