use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, Identifier, Locate, NodeEvent, PortDirection, RefNode, SyntaxTree};

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

                let id: Locate = match unwrap_node!(*x, Identifier) {
                    Some(RefNode::Identifier(Identifier::SimpleIdentifier(_id))) => {
                        Some(_id.nodes.0)
                    }
                    Some(RefNode::Identifier(Identifier::EscapedIdentifier(_id))) => {
                        Some(_id.nodes.0)
                    }
                    _ => None,
                }
                .unwrap();
                let nm: &str = syntax_tree.get_str(&id).unwrap();
                let is_prefixed: bool = nm.starts_with(&option.prefix_input);

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
            "`input` must have prefix \"{}\"",
            &option.prefix_input
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
