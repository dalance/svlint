use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, Identifier, Locate, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixOutput;

impl Rule for PrefixOutput {
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
                let is_output: bool = match dir {
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => true,
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
                let is_prefixed: bool = nm.starts_with(&option.prefix_output);

                match (is_output, is_prefixed) {
                    (true, false) => RuleResult::Fail,
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_output")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!("`output` must have prefix \"{}\"", &option.prefix_output))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}