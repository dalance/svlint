use crate::config::ConfigOption;
use crate::linter::{check_prefix, SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixOutput;

impl SyntaxRule for PrefixOutput {
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
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_output: bool = match dir {
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => true,
                    _ => false,
                };

                if is_output {
                    check_prefix(unwrap_node!(*x, PortIdentifier), &syntax_tree, &option.prefix_output)
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_output")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Prefix `output` port identifier with \"{}\".",
            &option.prefix_output
        ))
    }

    fn reason(&self) -> String {
        String::from("Port prefixes help readers to follow signals through modules.")
    }
}
